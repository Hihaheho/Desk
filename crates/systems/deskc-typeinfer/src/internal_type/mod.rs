pub mod effect_expr;

use std::collections::HashMap;

use ids::NodeId;

use self::effect_expr::EffectExpr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Handler {
    pub input: Type,
    pub output: Type,
}

pub type Id = usize;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Effect {
    pub input: Type,
    pub output: Type,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Type {
    Real,
    Rational,
    Integer,
    String,
    Product(Vec<Self>),
    Sum(Vec<Self>),
    Function {
        parameter: Box<Self>,
        body: Box<Self>,
    },
    Vector(Box<Self>),
    Map {
        key: Box<Self>,
        value: Box<Self>,
    },
    Variable(Id),
    ForAll {
        variable: Id,
        bound: Option<Box<Self>>,
        body: Box<Self>,
    },
    Existential(Id),
    Infer(NodeId),
    Effectful {
        ty: Box<Self>,
        effects: EffectExpr,
    },
    Brand {
        brand: String,
        item: Box<Self>,
    },
    Label {
        label: String,
        item: Box<Self>,
    },
}

pub(crate) trait TypeVisitorMut {
    fn visit_real(&mut self) {}
    fn visit_rational(&mut self) {}
    fn visit_integer(&mut self) {}
    fn visit_string(&mut self) {}
    fn visit_product(&mut self, types: &mut Vec<Type>) {
        types.iter_mut().for_each(|ty| self.visit(ty))
    }
    fn visit_sum(&mut self, types: &mut Vec<Type>) {
        types.iter_mut().for_each(|ty| self.visit(ty))
    }
    fn visit_function(&mut self, parameter: &mut Type, body: &mut Type) {
        self.visit(parameter);
        self.visit(body);
    }
    fn visit_array(&mut self, ty: &mut Type) {
        self.visit(ty);
    }
    fn visit_map(&mut self, key: &mut Type, value: &mut Type) {
        self.visit(key);
        self.visit(value);
    }
    fn visit_variable(&mut self, _id: &mut Id) {}
    fn visit_forall(&mut self, _variable: &mut Id, bound: &mut Option<Box<Type>>, body: &mut Type) {
        if let Some(bound) = bound {
            self.visit(bound);
        }
        self.visit(body);
    }
    fn visit_existential(&mut self, _id: &mut Id) {}
    fn visit_infer(&mut self, _id: &mut NodeId) {}
    fn visit_effectful(&mut self, ty: &mut Type, effects: &mut EffectExpr) {
        self.visit(ty);
        self.visit_effect_expr(effects);
    }
    fn visit_effect_expr(&mut self, effects: &mut EffectExpr) {
        match effects {
            EffectExpr::Effects(effects) => {
                self.visit_effect_expr_effects(effects);
            }
            EffectExpr::Add(effects) => {
                self.visit_effect_expr_add(effects);
            }
            EffectExpr::Sub {
                minuend,
                subtrahend,
            } => {
                self.visit_effect_expr_sub(minuend, subtrahend);
            }
            EffectExpr::Apply {
                function,
                arguments,
            } => {
                self.visit_effect_expr_apply(function, arguments);
            }
        }
    }
    fn visit_effect_expr_effects(&mut self, effects: &mut Vec<Effect>) {
        effects
            .iter_mut()
            .for_each(|effect| self.visit_effect(effect))
    }
    fn visit_effect_expr_add(&mut self, exprs: &mut Vec<EffectExpr>) {
        exprs
            .iter_mut()
            .for_each(|expr| self.visit_effect_expr(expr))
    }
    fn visit_effect_expr_sub(&mut self, minuend: &mut EffectExpr, subtrahend: &mut EffectExpr) {
        self.visit_effect_expr(minuend);
        self.visit_effect_expr(subtrahend);
    }
    fn visit_effect_expr_apply(&mut self, function: &mut Type, arguments: &mut Vec<Type>) {
        self.visit(function);
        arguments.iter_mut().for_each(|arg| self.visit(arg));
    }
    fn visit_effect(&mut self, effect: &mut Effect) {
        self.visit(&mut effect.input);
        self.visit(&mut effect.output);
    }
    fn visit_brand(&mut self, _brand: &mut String, item: &mut Type) {
        self.visit(item);
    }
    fn visit_label(&mut self, _label: &mut String, item: &mut Type) {
        self.visit(item);
    }
    fn visit(&mut self, ty: &mut Type) {
        self.visit_inner(ty)
    }
    fn visit_inner(&mut self, ty: &mut Type) {
        match ty {
            Type::Real => self.visit_real(),
            Type::Rational => self.visit_rational(),
            Type::Integer => self.visit_integer(),
            Type::String => self.visit_string(),
            Type::Product(types) => self.visit_product(types),
            Type::Sum(types) => self.visit_sum(types),
            Type::Function { parameter, body } => self.visit_function(parameter, body),
            Type::Vector(ty) => self.visit_array(ty),
            Type::Map { key, value } => self.visit_map(key, value),
            Type::Variable(id) => self.visit_variable(id),
            Type::ForAll {
                variable,
                bound,
                body,
            } => self.visit_forall(variable, bound, body),
            Type::Existential(id) => self.visit_existential(id),
            Type::Infer(id) => self.visit_infer(id),
            Type::Effectful { ty, effects } => self.visit_effectful(ty, effects),
            Type::Brand { brand, item } => self.visit_brand(brand, item),
            Type::Label { label, item } => self.visit_label(label, item),
        }
    }
}

pub(crate) trait TypeVisitor {
    fn visit_real(&mut self) {}
    fn visit_rational(&mut self) {}
    fn visit_integer(&mut self) {}
    fn visit_string(&mut self) {}
    fn visit_product(&mut self, types: &[Type]) {
        types.iter().for_each(|ty| self.visit(ty))
    }
    fn visit_sum(&mut self, types: &[Type]) {
        types.iter().for_each(|ty| self.visit(ty))
    }
    fn visit_function(&mut self, parameter: &Type, body: &Type) {
        self.visit(parameter);
        self.visit(body);
    }
    fn visit_array(&mut self, ty: &Type) {
        self.visit(ty);
    }
    fn visit_map(&mut self, key: &Type, value: &Type) {
        self.visit(key);
        self.visit(value);
    }
    fn visit_variable(&mut self, _id: &Id) {}
    fn visit_forall(&mut self, _variable: &Id, bound: &Option<Box<Type>>, body: &Type) {
        if let Some(bound) = bound {
            self.visit(bound);
        }
        self.visit(body);
    }
    fn visit_existential(&mut self, _id: &Id) {}
    fn visit_infer(&mut self, _id: &NodeId) {}
    fn visit_effectful(&mut self, ty: &Type, effects: &EffectExpr) {
        self.visit(ty);
        self.visit_effect_expr(effects);
    }
    fn visit_effect_expr(&mut self, effects: &EffectExpr) {
        match effects {
            EffectExpr::Effects(effects) => {
                self.visit_effect_expr_effects(effects);
            }
            EffectExpr::Add(effects) => {
                self.visit_effect_expr_add(effects);
            }
            EffectExpr::Sub {
                minuend,
                subtrahend,
            } => {
                self.visit_effect_expr_sub(minuend, subtrahend);
            }
            EffectExpr::Apply {
                function,
                arguments,
            } => {
                self.visit_effect_expr_apply(function, arguments);
            }
        }
    }
    fn visit_effect_expr_effects(&mut self, effects: &[Effect]) {
        effects.iter().for_each(|effect| self.visit_effect(effect))
    }
    fn visit_effect_expr_add(&mut self, exprs: &[EffectExpr]) {
        exprs.iter().for_each(|expr| self.visit_effect_expr(expr))
    }
    fn visit_effect_expr_sub(&mut self, minuend: &EffectExpr, subtrahend: &EffectExpr) {
        self.visit_effect_expr(minuend);
        self.visit_effect_expr(subtrahend);
    }
    fn visit_effect_expr_apply(&mut self, function: &Type, arguments: &[Type]) {
        self.visit(function);
        arguments.iter().for_each(|arg| self.visit(arg));
    }
    fn visit_effect(&mut self, effect: &Effect) {
        self.visit(&effect.input);
        self.visit(&effect.output);
    }
    fn visit_brand(&mut self, _brand: &str, item: &Type) {
        self.visit(item);
    }
    fn visit_label(&mut self, _label: &str, item: &Type) {
        self.visit(item);
    }
    fn visit(&mut self, ty: &Type) {
        self.visit_inner(ty)
    }
    fn visit_inner(&mut self, ty: &Type) {
        match ty {
            Type::Real => self.visit_real(),
            Type::Rational => self.visit_rational(),
            Type::Integer => self.visit_integer(),
            Type::String => self.visit_string(),
            Type::Product(types) => self.visit_product(types),
            Type::Sum(types) => self.visit_sum(types),
            Type::Function { parameter, body } => self.visit_function(parameter, body),
            Type::Vector(ty) => self.visit_array(ty),
            Type::Map { key, value } => self.visit_map(key, value),
            Type::Variable(id) => self.visit_variable(id),
            Type::ForAll {
                variable,
                bound,
                body,
            } => self.visit_forall(variable, bound, body),
            Type::Existential(id) => self.visit_existential(id),
            Type::Infer(id) => self.visit_infer(id),
            Type::Effectful { ty, effects } => self.visit_effectful(ty, effects),
            Type::Brand { brand, item } => self.visit_brand(brand, item),
            Type::Label { label, item } => self.visit_label(label, item),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ExprTypes {
    pub types: HashMap<Id, Type>,
}
