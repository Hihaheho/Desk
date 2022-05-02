use crate::substitute::substitute;

use super::{Effect, Type, TypeVisitorMut};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EffectExpr {
    Effects(Vec<Effect>),
    Add(Vec<EffectExpr>),
    Sub {
        minuend: Box<EffectExpr>,
        subtrahend: Box<EffectExpr>,
    },
    Apply {
        function: Box<Type>,
        arguments: Vec<Type>,
    },
}

pub fn simplify(ty: &mut Type) {
    EffectExprSimplifier.visit(ty);
}

pub fn simplify_effect_expr(expr: &mut EffectExpr) {
    EffectExprSimplifier.visit_effect_expr(expr);
}

struct EffectExprSimplifier;

impl TypeVisitorMut for EffectExprSimplifier {
    fn visit_effectful(&mut self, ty: &mut Type, effects: &mut EffectExpr) {
        self.visit(ty);
        self.visit_effect_expr(effects);
        if let Type::Effectful {
            ty: inner_ty,
            effects: inner_effects,
        } = ty
        {
            let mut added = EffectExpr::Add(vec![effects.clone(), inner_effects.clone()]);
            self.visit_effect_expr(&mut added);
            *ty = *inner_ty.clone();
            *effects = added;
        }
    }
    fn visit_effect_expr_effects(&mut self, effects: &mut Vec<Effect>) {
        for effect in effects.iter_mut() {
            self.visit_effect(effect);
        }
        effects.sort();
        effects.dedup();
    }
    fn visit_effect_expr_add(&mut self, exprs: &mut Vec<EffectExpr>) {
        for expr in exprs.iter_mut() {
            self.visit_effect_expr(expr);
        }
        let mut collected_effects = Vec::new();
        let drain = exprs.drain(..).collect::<Vec<_>>();
        for expr in drain {
            match expr {
                EffectExpr::Effects(effects) => collected_effects.extend(effects),
                other => exprs.push(other),
            }
        }
        let mut collected_effects = EffectExpr::Effects(collected_effects);
        self.visit_effect_expr(&mut collected_effects);
        exprs.push(collected_effects);
        exprs.retain(|effect| !effect.is_empty());
        exprs.sort();
        exprs.dedup();
    }
    fn visit_effect_expr_sub(&mut self, minuend: &mut EffectExpr, subtrahend: &mut EffectExpr) {
        self.visit_effect_expr(minuend);
        self.visit_effect_expr(subtrahend);
        if let (EffectExpr::Effects(minuend), EffectExpr::Effects(subtrahend)) =
            (minuend, subtrahend)
        {
            minuend.retain(|e| !subtrahend.contains(e));
            subtrahend.truncate(0);
        }
    }
    fn visit_effect_expr_apply(&mut self, function: &mut Type, arguments: &mut Vec<Type>) {
        self.visit(function);
        for argument in arguments.iter_mut() {
            self.visit(argument);
        }
        if arguments.is_empty() {
            return;
        }
        match function {
            Type::ForAll { variable: _, body } => {
                *function = *body.clone();
                self.visit_effect_expr_apply(function, arguments)
            }
            Type::Function { parameter, body } => {
                let popped = arguments.remove(0);
                match &**parameter {
                    Type::Variable(id) => {
                        *function = substitute(body, id, &popped);
                    }
                    Type::Existential(id) => {
                        *function = substitute(body, id, &popped);
                    }
                    _ => {
                        *function = *body.clone();
                    }
                }
                self.visit_effect_expr_apply(function, arguments)
            }
            _ => {}
        }
    }
    fn visit_effect_expr(&mut self, effects: &mut EffectExpr) {
        match effects {
            EffectExpr::Effects(effects) => {
                self.visit_effect_expr_effects(effects);
            }
            EffectExpr::Add(exprs) => {
                self.visit_effect_expr_add(exprs);
                if exprs.len() == 1 {
                    *effects = exprs[0].clone();
                }
            }
            EffectExpr::Sub {
                minuend,
                subtrahend,
            } => {
                self.visit_effect_expr_sub(minuend, subtrahend);
                if minuend.is_empty() || subtrahend.is_empty() {
                    *effects = *minuend.clone();
                }
            }
            EffectExpr::Apply {
                function,
                arguments,
            } => {
                self.visit_effect_expr_apply(function, arguments);
                if arguments.is_empty() {
                    match &**function {
                        Type::Effectful {
                            ty: _,
                            effects: expr,
                        } => {
                            *effects = expr.clone();
                        }
                        _ => *effects = EffectExpr::Effects(vec![]),
                    }
                }
            }
        }
    }
}

impl EffectExpr {
    pub fn is_empty(&self) -> bool {
        match self {
            EffectExpr::Effects(effects) => effects.is_empty(),
            EffectExpr::Add(effects) => effects.is_empty(),
            EffectExpr::Sub { .. } => false,
            EffectExpr::Apply { .. } => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ty::{Effect, Type};

    use super::*;

    #[test]
    fn simplify_effects() {
        let mut effects = EffectExpr::Effects(vec![
            Effect {
                input: Type::Number,
                output: Type::String,
            },
            Effect {
                input: Type::Number,
                output: Type::String,
            },
        ]);
        EffectExprSimplifier.visit_effect_expr(&mut effects);
        assert_eq!(
            effects,
            EffectExpr::Effects(vec![Effect {
                input: Type::Number,
                output: Type::String,
            }])
        );
    }

    #[test]
    fn simplify_add() {
        let mut effects = EffectExpr::Add(vec![
            EffectExpr::Effects(vec![Effect {
                input: Type::Number,
                output: Type::String,
            }]),
            EffectExpr::Effects(vec![]),
        ]);
        EffectExprSimplifier.visit_effect_expr(&mut effects);
        assert_eq!(
            effects,
            EffectExpr::Effects(vec![Effect {
                input: Type::Number,
                output: Type::String,
            }])
        );
    }

    #[test]
    fn simplify_sub() {
        let mut effects = EffectExpr::Sub {
            minuend: Box::new(EffectExpr::Effects(vec![Effect {
                input: Type::Number,
                output: Type::String,
            }])),
            subtrahend: Box::new(EffectExpr::Effects(vec![Effect {
                input: Type::Number,
                output: Type::String,
            }])),
        };
        EffectExprSimplifier.visit_effect_expr(&mut effects);
        assert_eq!(effects, EffectExpr::Effects(vec![]));
    }

    #[test]
    fn simplify_apply() {
        let mut effects = EffectExpr::Apply {
            function: Box::new(Type::ForAll {
                variable: 1,
                body: Box::new(Type::Function {
                    parameter: Box::new(Type::String),
                    body: Box::new(Type::Function {
                        parameter: Box::new(Type::Variable(1)),
                        body: Box::new(Type::Effectful {
                            ty: Box::new(Type::Number),
                            effects: EffectExpr::Add(vec![
                                EffectExpr::Effects(vec![Effect {
                                    input: Type::Number,
                                    output: Type::String,
                                }]),
                                EffectExpr::Apply {
                                    function: Box::new(Type::Variable(1)),
                                    arguments: vec![Type::Number],
                                },
                            ]),
                        }),
                    }),
                }),
            }),
            arguments: vec![
                Type::String,
                Type::Function {
                    parameter: Box::new(Type::Number),
                    body: Box::new(Type::Effectful {
                        ty: Box::new(Type::String),
                        effects: EffectExpr::Effects(vec![Effect {
                            input: Type::String,
                            output: Type::Number,
                        }]),
                    }),
                },
            ],
        };
        EffectExprSimplifier.visit_effect_expr(&mut effects);
        assert_eq!(
            effects,
            EffectExpr::Effects(vec![
                Effect {
                    input: Type::Number,
                    output: Type::String,
                },
                Effect {
                    input: Type::String,
                    output: Type::Number,
                }
            ])
        );
    }
}
