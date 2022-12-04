use errors::typeinfer::TypeError;

use crate::{
    ctx::{Ctx, Id, Log},
    mono_type::is_monotype,
    substitute::substitute,
    ty::{effect_expr::EffectExpr, Type},
};

impl Ctx {
    pub fn instantiate_supertype(&self, sub: &Type, id: &Id) -> Result<Ctx, TypeError> {
        // In here, we can assume the context contains the existential type.
        let ctx = if is_monotype(sub)
            && self.has_existential(id)
            && self
                .truncate_from(&Log::Existential(*id))
                .recover_effects()
                .is_well_formed(sub)
        {
            self.insert_in_place(&Log::Existential(*id), vec![Log::Solved(*id, sub.clone())])
        } else {
            match sub {
                Type::Effectful { ty, effects } => {
                    self.instantiate_supertype(ty, id)?.add_effects(effects)
                }
                Type::Function { parameter, body } => {
                    let a1 = self.fresh_existential();
                    let a2 = self.fresh_existential();
                    let theta = self
                        .insert_in_place(
                            &Log::Existential(*id),
                            vec![
                                Log::Existential(a2),
                                Log::Existential(a1),
                                Log::Solved(
                                    *id,
                                    Type::Function {
                                        parameter: Box::new(Type::Existential(a1)),
                                        body: Box::new(Type::Existential(a2)),
                                    },
                                ),
                            ],
                        )
                        .instantiate_subtype(&a1, parameter)?;
                    theta.instantiate_supertype(&theta.substitute_from_ctx(body), &a2)?
                }
                Type::ForAll {
                    variable,
                    bound,
                    body,
                } => self
                    .add(Log::Marker(*variable))
                    .add(Log::Existential(*variable))
                    .instantiate_supertype(
                        &substitute(body, variable, &Type::Existential(*variable)),
                        id,
                    )?
                    .bound_check(&Type::Variable(*variable), bound)?
                    .truncate_from(&Log::Marker(*variable))
                    .recover_effects(),
                Type::Existential(a) => self.insert_in_place(
                    &Log::Existential(*id),
                    vec![Log::Solved(*id, Type::Existential(*a))],
                ),
                Type::Product(types) => self.instantiate_composite_type_vec(
                    *id,
                    types,
                    Type::Product,
                    |ctx, id, sub| ctx.instantiate_supertype(sub, id),
                )?,
                Type::Sum(types) => {
                    self.instantiate_composite_type_vec(*id, types, Type::Sum, |ctx, id, sub| {
                        ctx.instantiate_supertype(sub, id)
                    })?
                }
                Type::Vector(ty) => {
                    let a = self.fresh_existential();
                    self.insert_in_place(
                        &Log::Existential(*id),
                        vec![
                            Log::Existential(a),
                            Log::Solved(*id, Type::Vector(Box::new(Type::Existential(a)))),
                        ],
                    )
                    .instantiate_supertype(ty, &a)?
                }
                Type::Map { key, value } => {
                    let k = self.fresh_existential();
                    let v = self.fresh_existential();
                    self.insert_in_place(
                        &Log::Existential(*id),
                        vec![
                            Log::Existential(k),
                            Log::Existential(v),
                            Log::Solved(
                                *id,
                                Type::Map {
                                    key: Box::new(Type::Existential(k)),
                                    value: Box::new(Type::Existential(v)),
                                },
                            ),
                        ],
                    )
                    .instantiate_supertype(key, &k)?
                    .instantiate_supertype(value, &v)?
                }
                Type::Label { item, label } => {
                    let a = self.fresh_existential();
                    self.insert_in_place(
                        &Log::Existential(*id),
                        vec![
                            Log::Existential(a),
                            Log::Solved(
                                *id,
                                Type::Label {
                                    item: Box::new(Type::Existential(a)),
                                    label: label.clone(),
                                },
                            ),
                        ],
                    )
                    .instantiate_supertype(item, &a)?
                }
                Type::Brand { item, brand } => {
                    let a = self.fresh_existential();
                    self.insert_in_place(
                        &Log::Existential(*id),
                        vec![
                            Log::Existential(a),
                            Log::Solved(
                                *id,
                                Type::Brand {
                                    item: Box::new(Type::Existential(a)),
                                    brand: brand.clone(),
                                },
                            ),
                        ],
                    )
                    .instantiate_supertype(item, &a)?
                }
                Type::Infer(infer) => {
                    self.store_inferred_type(infer.clone(), Type::Existential(*id));
                    self.insert_in_place(
                        &Log::Existential(*id),
                        vec![Log::Solved(*id, sub.clone())],
                    )
                }
                ty => return Err(TypeError::NotInstantiableSupertype { ty: self.gen_type(ty) }),
            }
        };
        self.store_solved_type_and_effects(*id, sub.clone(), EffectExpr::Effects(vec![]));
        Ok(ctx)
    }
}
