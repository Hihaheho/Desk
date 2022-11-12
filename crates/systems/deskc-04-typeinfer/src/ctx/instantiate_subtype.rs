use crate::{
    ctx::{Ctx, Id, Log},
    error::TypeError,
    mono_type::is_monotype,
    ty::{effect_expr::EffectExpr, Type},
};

impl Ctx {
    pub fn instantiate_subtype(&self, id: &Id, sup: &Type) -> Result<Ctx, TypeError> {
        // In here, we can assume the context contains the existential type.
        let ctx = if is_monotype(sup)
            && self.has_existential(id)
            && self
                .truncate_from(&Log::Existential(*id))
                .recover_effects()
                .is_well_formed(sup)
        {
            self.insert_in_place(&Log::Existential(*id), vec![Log::Solved(*id, sup.clone())])
        } else {
            match sup {
                Type::Effectful { ty, effects: _ } => self.instantiate_subtype(id, ty)?,
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
                        .instantiate_supertype(parameter, &a1)?;
                    theta.instantiate_subtype(&a2, &theta.substitute_from_ctx(body))?
                }
                Type::ForAll { variable, body } => self
                    .add(Log::Variable(*variable))
                    .instantiate_subtype(id, body)?
                    .truncate_from(&Log::Variable(*variable))
                    .recover_effects(),
                Type::Existential(b) => self.insert_in_place(
                    &Log::Existential(*b),
                    vec![Log::Solved(*b, Type::Existential(*id))],
                ),
                Type::Product(types) => self.instantiate_composite_type_vec(
                    *id,
                    types,
                    Type::Product,
                    |ctx, id, sup| ctx.instantiate_subtype(id, sup),
                )?,
                Type::Sum(types) => {
                    self.instantiate_composite_type_vec(*id, types, Type::Sum, |ctx, id, sup| {
                        ctx.instantiate_subtype(id, sup)
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
                    .instantiate_subtype(&a, ty)?
                }
                Type::Set(ty) => {
                    let a = self.fresh_existential();
                    self.insert_in_place(
                        &Log::Existential(*id),
                        vec![
                            Log::Existential(a),
                            Log::Solved(*id, Type::Set(Box::new(Type::Existential(a)))),
                        ],
                    )
                    .instantiate_subtype(&a, ty)?
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
                    .instantiate_subtype(&a, item)?
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
                    .instantiate_subtype(&a, item)?
                }
                Type::Infer(infer) => {
                    self.store_inferred_type(infer.clone(), Type::Existential(*id));
                    self.insert_in_place(
                        &Log::Existential(*id),
                        vec![Log::Solved(*id, sup.clone())],
                    )
                }
                ty => return Err(TypeError::NotInstantiableSubtype { ty: ty.clone() }),
            }
        };
        self.store_solved_type_and_effects(*id, sup.clone(), EffectExpr::Effects(vec![]));
        Ok(ctx)
    }
}
