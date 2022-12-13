use errors::typeinfer::TypeError;

use crate::{
    cast_strategies::CastStrategy,
    ctx::Ctx,
    ctx::Log,
    internal_type::{effect_expr::EffectExpr, Type},
    occurs_in::occurs_in,
    partial_ord_max::PartialOrdMax,
    similarity::{SimilaritiesList, Similarity, WithSimilarities, WithSimilaritiesList},
    substitute::substitute,
};

pub(crate) type CtxWithMappings<'a> = (Ctx, Vec<(&'a Type, &'a Type)>);

impl Ctx {
    pub fn subtype(&self, sub: &Type, ty: &Type) -> Result<WithSimilarities<Ctx>, TypeError> {
        let result = match (sub, ty) {
            (ty1, ty2) if ty1 == ty2 => self
                .clone()
                .with_similarities(vec![Similarity::Same].into()),
            (Type::Integer, Type::Rational) => self
                .clone()
                .with_similarities(vec![Similarity::Number].into()),
            (Type::Integer, Type::Real) => self
                .clone()
                .with_similarities(vec![Similarity::Number].into()),
            (Type::Rational, Type::Real) => self
                .clone()
                .with_similarities(vec![Similarity::Number].into()),
            (Type::Existential(id), ty) => {
                if occurs_in(id, ty) {
                    return Err(TypeError::CircularExistential {
                        id: *id,
                        ty: self.gen_type_or_string(ty),
                    });
                } else {
                    self.instantiate_subtype(id, ty)?
                        .with_similarities(vec![Similarity::Instantiate].into())
                }
            }
            (sub, Type::Existential(id)) => {
                if occurs_in(id, sub) {
                    return Err(TypeError::CircularExistential {
                        id: *id,
                        ty: self.gen_type_or_string(ty),
                    });
                } else {
                    self.instantiate_supertype(sub, id)?
                        .with_similarities(vec![Similarity::Instantiate].into())
                }
            }

            // handling things must be under the instantiations of existential.
            (Type::Product(sub_types), ty) => {
                let mut candidates: Vec<_> = sub_types
                    .iter()
                    .filter_map(|sub_ty| match self.subtype(sub_ty, ty) {
                        Ok(ctx) => Some((sub_ty, ctx)),
                        Err(_) => None,
                    })
                    .collect();
                candidates.sort_by_key(|(_, ctx)| ctx.similarities.clone());
                candidates.reverse();
                let inner = if candidates.is_empty() {
                    Err(TypeError::NotSubtype {
                        sub: self.gen_type_or_string(sub),
                        ty: self.gen_type_or_string(ty),
                    })
                } else if candidates.len() == 1
                    || candidates[0].1.similarities > candidates[1].1.similarities
                {
                    let (sub_ty, ctx) = candidates[0].clone();
                    ctx.ctx.cast_strategies.borrow_mut().insert(
                        (sub.clone(), ty.clone()),
                        CastStrategy::ProductToInner(sub_ty.clone()),
                    );
                    Ok(ctx.insert_similarity(Similarity::ProductToInner))
                } else {
                    Err(TypeError::AmbiguousSubtype {
                        sub: self.gen_type_or_string(sub),
                        ty: self.gen_type_or_string(ty),
                    })
                };
                if let Ok(inner) = inner {
                    return Ok(inner);
                }
                if let Type::Product(types) = ty {
                    let mappings = self
                        .all_product_mappings(sub_types.iter().collect(), types.iter().collect())?;
                    if let Some(max) = self.max_mappings(mappings) {
                        let (ctx, mapping) = max.ctx;
                        ctx.cast_strategies.borrow_mut().insert(
                            (sub.clone(), ty.clone()),
                            CastStrategy::ProductToProduct(
                                mapping
                                    .into_iter()
                                    .map(|(l, r)| (l.clone(), r.clone()))
                                    .collect(),
                            ),
                        );
                        let mut similarities = max.list.max();
                        similarities.insert(Similarity::Product);
                        ctx.with_similarities(similarities)
                    } else {
                        return Err(TypeError::NotSubtype {
                            sub: self.gen_type_or_string(sub),
                            ty: self.gen_type_or_string(ty),
                        });
                    }
                } else {
                    return inner;
                }
            }
            (sub, Type::Sum(types)) => {
                let mut candidates: Vec<_> = types
                    .iter()
                    .filter_map(|ty| match self.subtype(sub, ty) {
                        Ok(ctx) => Some((ty, ctx)),
                        Err(_) => None,
                    })
                    .collect();
                candidates.sort_by_key(|(_, ctx)| ctx.similarities.clone());
                candidates.reverse();
                let inner = if candidates.is_empty() {
                    Err(TypeError::NotSubtype {
                        sub: self.gen_type_or_string(sub),
                        ty: self.gen_type_or_string(ty),
                    })
                } else if candidates.len() == 1
                    || candidates[0].1.similarities > candidates[1].1.similarities
                {
                    let (first, ctx) = candidates[0].clone();
                    ctx.ctx.cast_strategies.borrow_mut().insert(
                        (sub.clone(), ty.clone()),
                        CastStrategy::InnerToSum(first.clone()),
                    );
                    Ok(ctx.insert_similarity(Similarity::InnerToSum))
                } else {
                    Err(TypeError::AmbiguousSubtype {
                        sub: self.gen_type_or_string(sub),
                        ty: self.gen_type_or_string(ty),
                    })
                };
                if let Ok(inner) = inner {
                    return Ok(inner);
                }
                if let Type::Sum(sub_types) = sub {
                    let mappings =
                        self.all_sum_mappings(sub_types.iter().collect(), types.iter().collect())?;
                    if let Some(max) = self.max_mappings(mappings) {
                        let (ctx, mapping) = max.ctx;
                        ctx.cast_strategies.borrow_mut().insert(
                            (sub.clone(), ty.clone()),
                            CastStrategy::SumToSum(
                                mapping
                                    .into_iter()
                                    .map(|(l, r)| (l.clone(), r.clone()))
                                    .collect(),
                            ),
                        );
                        let mut similarities = max.list.max();
                        similarities.insert(Similarity::Sum);
                        ctx.with_similarities(similarities)
                    } else {
                        return Err(TypeError::NotSubtype {
                            sub: self.gen_type_or_string(sub),
                            ty: self.gen_type_or_string(ty),
                        });
                    }
                } else {
                    return inner;
                }
            }
            (
                Type::Function {
                    parameter: sub_parameter,
                    body: sub_body,
                },
                Type::Function { parameter, body },
            ) => {
                let theta = self.subtype(parameter, sub_parameter)?.ctx;
                theta
                    .subtype(
                        &theta.substitute_from_ctx(sub_body),
                        &theta.substitute_from_ctx(body),
                    )?
                    .insert_similarity(Similarity::Function)
            }
            (Type::Vector(sub), Type::Vector(ty)) => {
                self.subtype(sub, ty)?.insert_similarity(Similarity::Vector)
            }
            (
                Type::Map {
                    key: sub_key,
                    value: sub_value,
                },
                Type::Map { key, value },
            ) => {
                let WithSimilarities {
                    ctx,
                    similarities: key,
                } = self.subtype(sub_key, key)?;
                let WithSimilarities {
                    ctx,
                    similarities: value,
                } = ctx.subtype(sub_value, value)?;
                let mut max = key.max(value);
                max.insert(Similarity::Map);
                ctx.with_similarities(max)
            }
            (
                Type::ForAll {
                    variable,
                    bound,
                    body,
                },
                ty,
            ) => {
                let a = self.fresh_existential();
                self.add(Log::Marker(a))
                    .add(Log::Existential(a))
                    .subtype(&substitute(body, variable, &Type::Existential(a)), ty)?
                    .ctx_do(|ctx| {
                        // TODO: is this correct bound check?
                        Ok(ctx
                            .bound_check(&Type::Existential(a), bound)?
                            .truncate_from(&Log::Marker(a))
                            .recover_effects())
                    })?
            }
            (
                sub,
                Type::ForAll {
                    variable,
                    bound,
                    body,
                },
            ) => self
                .add(Log::Variable(*variable))
                .subtype(sub, body)?
                .ctx_do(|ctx|
                // TODO: is this correct bound check?
                Ok(ctx.bound_check(&Type::Variable(*variable), bound)?
                .truncate_from(&Log::Variable(*variable))
                .recover_effects()))?,
            (
                Type::Label { label, item },
                Type::Label {
                    label: label2,
                    item: item2,
                },
            ) => {
                let ret = self.subtype(item, item2)?;
                if label == label2 {
                    ret.insert_similarity(Similarity::LabelMatch)
                } else {
                    ret.insert_similarity(Similarity::LabelMismatch)
                }
            }
            (
                Type::Brand { brand, item },
                Type::Brand {
                    brand: brand2,
                    item: item2,
                },
            ) if brand == brand2 => self
                .subtype(item, item2)?
                .insert_similarity(Similarity::BrandMatch),
            (sub, Type::Label { item, label: _ }) => self
                .subtype(sub, item)?
                .insert_similarity(Similarity::InnerToLabel),
            (Type::Label { item, label: _ }, sup) => self
                .subtype(item, sup)?
                .insert_similarity(Similarity::LabelToInner),
            // one without brand is not subtype of other with brand
            (Type::Brand { item, brand: _ }, sup) => self
                .subtype(item, sup)?
                .insert_similarity(Similarity::BrandToInner),
            (Type::Infer(id), sup) => {
                self.store_inferred_type(id.clone(), sup.clone());
                self.clone()
                    .with_similarities(vec![Similarity::Infer].into())
            }
            (sub, Type::Infer(id)) => {
                self.store_inferred_type(id.clone(), sub.clone());
                self.clone()
                    .with_similarities(vec![Similarity::Infer].into())
            }

            (
                Type::Effectful { ty, effects },
                Type::Effectful {
                    ty: ty2,
                    effects: super_effects,
                },
            ) => {
                let theta = self.subtype(ty, ty2)?;
                theta.ctx_do(|ctx| {
                    Ok(ctx.add_effects(&EffectExpr::Sub {
                        minuend: Box::new(effects.clone()),
                        subtrahend: Box::new(super_effects.clone()),
                    }))
                })?
            }
            (Type::Effectful { ty, effects }, ty2) => {
                let theta = self.subtype(ty, ty2)?;
                theta.ctx_do(|ctx| Ok(ctx.add_effects(effects)))?
            }
            (sub, Type::Effectful { ty, effects: _ }) => self.subtype(sub, ty)?,
            (_, _) => {
                return Err(TypeError::NotSubtype {
                    sub: self.gen_type_or_string(sub),
                    ty: self.gen_type_or_string(ty),
                })
            }
        };
        Ok(result)
    }

    // This is a helper function for bound check.
    pub fn bound_check(&self, sub: &Type, bound: &Option<Box<Type>>) -> Result<Self, TypeError> {
        if let Some(bound) = bound {
            self.subtype(sub, bound).map(|with| with.ctx)
        } else {
            Ok(self.clone())
        }
    }

    fn max_mappings<'a>(
        &self,
        mappings: Vec<Vec<(&'a Type, &'a Type)>>,
    ) -> Option<WithSimilaritiesList<CtxWithMappings<'a>>> {
        let candidates: Vec<_> = mappings
            .into_iter()
            .filter_map(|mapping| {
                mapping
                    .iter()
                    .try_fold::<_, _, Result<_, TypeError>>(
                        (self.clone(), SimilaritiesList(vec![])),
                        |(ctx, mut results), (sub, ty)| {
                            let WithSimilarities { ctx, similarities } = ctx.subtype(sub, ty)?;
                            results.push(similarities);
                            Ok((ctx, results))
                        },
                    )
                    .ok()
                    .map(|(ctx, list)| WithSimilaritiesList {
                        ctx: (ctx, mapping),
                        list,
                    })
            })
            .collect();
        candidates.into_iter().partial_max()
    }
}

#[cfg(test)]
mod tests {
    use ids::NodeId;

    use super::*;

    #[test]
    fn test_subtype_of_same() {
        let ctx = Ctx::default();
        assert_eq!(
            ctx.subtype(&Type::Variable(0), &Type::Variable(0))
                .map(|with| with.similarities),
            Ok(vec![Similarity::Same].into())
        );
    }

    #[test]
    fn test_subtype_number_ok() {
        let ctx = Ctx::default();
        assert_eq!(
            ctx.subtype(&Type::Integer, &Type::Rational)
                .map(|with| with.similarities),
            Ok(vec![Similarity::Number].into())
        );
        assert_eq!(
            ctx.subtype(&Type::Integer, &Type::Real)
                .map(|with| with.similarities),
            Ok(vec![Similarity::Number].into())
        );
        assert_eq!(
            ctx.subtype(&Type::Integer, &Type::Rational)
                .map(|with| with.similarities),
            Ok(vec![Similarity::Number].into())
        );
    }

    #[test]
    fn test_subtype_number_err() {
        let ctx = Ctx::default();
        assert_eq!(
            ctx.subtype(&Type::Rational, &Type::Integer),
            Err(TypeError::NotSubtype {
                sub: ty::Type::Rational.into(),
                ty: ty::Type::Integer.into(),
            })
        );
        assert_eq!(
            ctx.subtype(&Type::Real, &Type::Integer),
            Err(TypeError::NotSubtype {
                sub: ty::Type::Real.into(),
                ty: ty::Type::Integer.into(),
            })
        );
        assert_eq!(
            ctx.subtype(&Type::Real, &Type::Rational),
            Err(TypeError::NotSubtype {
                sub: ty::Type::Real.into(),
                ty: ty::Type::Rational.into(),
            })
        );
    }

    #[test]
    fn test_subtype_infer() {
        let ctx = Ctx::default();
        assert_eq!(
            ctx.subtype(&Type::Infer(NodeId::new()), &Type::Integer)
                .map(|with| with.similarities),
            Ok(vec![Similarity::Infer].into())
        );
        assert_eq!(
            ctx.subtype(&Type::Integer, &Type::Infer(NodeId::new()))
                .map(|with| with.similarities),
            Ok(vec![Similarity::Infer].into())
        );
    }

    #[test]
    fn test_subtype_product_to_inner() {
        let ctx = Ctx::default();
        let ret = ctx
            .subtype(
                &Type::Product(vec![Type::Integer, Type::Rational]),
                &Type::Integer,
            )
            .unwrap();
        assert_eq!(
            ret.similarities,
            vec![Similarity::ProductToInner, Similarity::Same].into()
        );
        assert_eq!(
            ret.ctx
                .cast_strategies
                .borrow()
                .get(&(
                    Type::Product(vec![Type::Integer, Type::Rational]),
                    Type::Integer
                ))
                .expect("cast strategy not found"),
            &CastStrategy::ProductToInner(Type::Integer)
        );
    }

    #[test]
    fn test_subtype_product_to_inner_product() {
        let ctx = Ctx::default();
        let ret = ctx
            .subtype(
                &Type::Product(vec![
                    Type::Product(vec![Type::Integer, Type::Rational]),
                    Type::Real,
                ]),
                &Type::Product(vec![Type::Integer, Type::Rational]),
            )
            .unwrap();
        assert_eq!(
            ret.similarities,
            vec![Similarity::ProductToInner, Similarity::Same,].into()
        );
        assert_eq!(
            ret.ctx
                .cast_strategies
                .borrow()
                .get(&(
                    Type::Product(vec![
                        Type::Product(vec![Type::Integer, Type::Rational]),
                        Type::Real,
                    ]),
                    Type::Product(vec![Type::Integer, Type::Rational])
                ))
                .expect("cast strategy not found"),
            &CastStrategy::ProductToInner(Type::Product(vec![Type::Integer, Type::Rational]))
        );
    }

    #[test]
    fn test_subtype_inner_to_sum() {
        let ctx = Ctx::default();
        let ret = ctx
            .subtype(
                &Type::Rational,
                &Type::Sum(vec![Type::Integer, Type::Rational]),
            )
            .unwrap();
        assert_eq!(
            ret.similarities,
            vec![Similarity::InnerToSum, Similarity::Same].into()
        );
        assert_eq!(
            ret.ctx
                .cast_strategies
                .borrow()
                .get(&(
                    Type::Rational,
                    Type::Sum(vec![Type::Integer, Type::Rational])
                ))
                .expect("cast strategy not found"),
            &CastStrategy::InnerToSum(Type::Rational)
        );
    }

    #[test]
    fn test_subtype_inner_sum_to_sum() {
        let ctx = Ctx::default();
        let ret = ctx
            .subtype(
                &Type::Sum(vec![Type::Integer, Type::Rational]),
                &Type::Sum(vec![
                    Type::Sum(vec![Type::Integer, Type::Rational]),
                    Type::Real,
                ]),
            )
            .unwrap();
        assert_eq!(
            ret.similarities,
            vec![Similarity::InnerToSum, Similarity::Same].into()
        );
        assert_eq!(
            ret.ctx
                .cast_strategies
                .borrow()
                .get(&(
                    Type::Sum(vec![Type::Integer, Type::Rational]),
                    Type::Sum(vec![
                        Type::Sum(vec![Type::Integer, Type::Rational]),
                        Type::Real,
                    ])
                ))
                .expect("cast strategy not found"),
            &CastStrategy::InnerToSum(Type::Sum(vec![Type::Integer, Type::Rational]))
        );
    }

    #[test]
    fn test_subtype_product_to_inner_ambiguous() {
        let ctx = Ctx::default();
        let ret = ctx.subtype(
            &Type::Product(vec![Type::Integer, Type::String]),
            &Type::Sum(vec![Type::Integer, Type::String]),
        );
        assert_eq!(
            ret,
            Err(TypeError::AmbiguousSubtype {
                sub: ty::Type::Product(vec![ty::Type::Integer, ty::Type::String]).into(),
                ty: ty::Type::Sum(vec![ty::Type::Integer, ty::Type::String]).into(),
            })
        );
    }

    #[test]
    fn test_subtype_inner_to_sum_ambiguous() {
        let ctx = Ctx::default();
        let ret = ctx.subtype(&Type::Integer, &Type::Sum(vec![Type::Rational, Type::Real]));
        assert_eq!(
            ret,
            Err(TypeError::AmbiguousSubtype {
                sub: ty::Type::Integer.into(),
                ty: ty::Type::Sum(vec![ty::Type::Rational, ty::Type::Real]).into(),
            })
        );
    }

    #[test]
    fn test_subtype_product_to_inner_most_similar() {
        let ctx = Ctx::default();
        let ret = ctx
            .subtype(
                &Type::Product(vec![Type::Integer, Type::Rational]),
                &Type::Rational,
            )
            .unwrap();
        assert_eq!(
            ret.similarities,
            vec![Similarity::ProductToInner, Similarity::Same].into()
        );
        assert_eq!(
            ret.ctx
                .cast_strategies
                .borrow()
                .get(&(
                    Type::Product(vec![Type::Integer, Type::Rational]),
                    Type::Rational
                ))
                .expect("cast strategy not found"),
            &CastStrategy::ProductToInner(Type::Rational)
        );
    }

    #[test]
    fn test_subtype_inner_to_sum_most_similar() {
        let ctx = Ctx::default();
        let ret = ctx
            .subtype(
                &Type::Integer,
                &Type::Sum(vec![Type::Integer, Type::Rational]),
            )
            .unwrap();
        assert_eq!(
            ret.similarities,
            vec![Similarity::InnerToSum, Similarity::Same].into()
        );
        assert_eq!(
            ret.ctx
                .cast_strategies
                .borrow()
                .get(&(
                    Type::Integer,
                    Type::Sum(vec![Type::Integer, Type::Rational])
                ))
                .expect("cast strategy not found"),
            &CastStrategy::InnerToSum(Type::Integer)
        );
    }

    #[test]
    fn test_subtype_product_to_product_insufficient() {
        let ctx = Ctx::default();
        let ret = ctx.subtype(
            &Type::Product(vec![Type::Integer, Type::Rational]),
            &Type::Product(vec![Type::Integer, Type::Rational, Type::Real]),
        );
        assert_eq!(
            ret,
            Err(TypeError::ProductInsufficentElements {
                sub_ty: vec![ty::Type::Integer.into(), ty::Type::Rational.into()],
                super_ty: vec![
                    ty::Type::Integer.into(),
                    ty::Type::Rational.into(),
                    ty::Type::Real.into()
                ],
            })
        );
    }

    #[test]
    fn test_subtype_sum_to_sum_insufficient() {
        let ctx = Ctx::default();
        let ret = ctx.subtype(
            &Type::Sum(vec![Type::Integer, Type::Rational, Type::Real]),
            &Type::Sum(vec![Type::Integer, Type::Rational]),
        );
        assert_eq!(
            ret,
            Err(TypeError::SumInsufficentElements {
                sub_ty: vec![
                    ty::Type::Integer.into(),
                    ty::Type::Rational.into(),
                    ty::Type::Real.into()
                ],
                super_ty: vec![ty::Type::Integer.into(), ty::Type::Rational.into(),],
            })
        );
    }

    #[test]
    fn test_subtype_product_to_product_most_similar() {
        let ctx = Ctx::default();
        let ret = ctx
            .subtype(
                &Type::Product(vec![Type::Integer, Type::Rational]),
                &Type::Product(vec![Type::Integer, Type::Real]),
            )
            .unwrap();
        assert_eq!(
            ret.similarities,
            vec![Similarity::Product, Similarity::Same].into()
        );
        assert_eq!(
            ret.ctx
                .cast_strategies
                .borrow()
                .get(&(
                    Type::Product(vec![Type::Integer, Type::Rational]),
                    Type::Product(vec![Type::Integer, Type::Real])
                ))
                .expect("cast strategy not found"),
            &CastStrategy::ProductToProduct(
                [(Type::Integer, Type::Integer), (Type::Rational, Type::Real)]
                    .into_iter()
                    .collect()
            )
        );
    }

    #[test]
    fn test_subtype_sum_to_sum_most_similar() {
        let ctx = Ctx::default();
        let ret = ctx
            .subtype(
                &Type::Sum(vec![Type::Integer, Type::Rational]),
                &Type::Sum(vec![Type::Integer, Type::Real]),
            )
            .unwrap();
        assert_eq!(
            ret.similarities,
            vec![Similarity::Sum, Similarity::Same].into()
        );
        assert_eq!(
            ret.ctx
                .cast_strategies
                .borrow()
                .get(&(
                    Type::Sum(vec![Type::Integer, Type::Rational]),
                    Type::Sum(vec![Type::Integer, Type::Real])
                ))
                .expect("cast strategy not found"),
            &CastStrategy::SumToSum(
                [(Type::Integer, Type::Integer), (Type::Rational, Type::Real)]
                    .into_iter()
                    .collect()
            )
        );
    }

    #[test]
    fn test_subtype_label_match() {
        let ctx = Ctx::default();
        let ret = ctx
            .subtype(
                &Type::Label {
                    label: "a".into(),
                    item: Box::new(Type::Integer),
                },
                &Type::Label {
                    label: "a".into(),
                    item: Box::new(Type::Real),
                },
            )
            .unwrap();
        assert_eq!(
            ret.similarities,
            vec![Similarity::LabelMatch, Similarity::Number].into()
        );
    }

    #[test]
    fn test_subtype_label_unmatch() {
        let ctx = Ctx::default();
        let ret = ctx
            .subtype(
                &Type::Label {
                    label: "a".into(),
                    item: Box::new(Type::Integer),
                },
                &Type::Label {
                    label: "b".into(),
                    item: Box::new(Type::Integer),
                },
            )
            .unwrap();
        assert_eq!(
            ret.similarities,
            vec![Similarity::LabelMismatch, Similarity::Same].into()
        );
    }

    #[test]
    fn test_subtype_brand_match() {
        let ctx = Ctx::default();
        let ret = ctx
            .subtype(
                &Type::Brand {
                    brand: "a".into(),
                    item: Box::new(Type::Integer),
                },
                &Type::Brand {
                    brand: "a".into(),
                    item: Box::new(Type::Real),
                },
            )
            .unwrap();
        assert_eq!(
            ret.similarities,
            vec![Similarity::BrandMatch, Similarity::Number].into()
        );
    }

    #[test]
    fn test_subtype_inner_to_label() {
        let ctx = Ctx::default();
        let ret = ctx
            .subtype(
                &Type::Integer,
                &Type::Label {
                    label: "a".into(),
                    item: Box::new(Type::Integer),
                },
            )
            .unwrap();
        assert_eq!(
            ret.similarities,
            vec![Similarity::InnerToLabel, Similarity::Same].into()
        );
    }

    #[test]
    fn test_subtype_label_to_inner() {
        let ctx = Ctx::default();
        let ret = ctx
            .subtype(
                &Type::Label {
                    label: "a".into(),
                    item: Box::new(Type::Integer),
                },
                &Type::Integer,
            )
            .unwrap();
        assert_eq!(
            ret.similarities,
            vec![Similarity::LabelToInner, Similarity::Same].into()
        );
    }

    #[test]
    fn test_subtype_brand_to_inner() {
        let ctx = Ctx::default();
        let ret = ctx
            .subtype(
                &Type::Brand {
                    brand: "a".into(),
                    item: Box::new(Type::Integer),
                },
                &Type::Integer,
            )
            .unwrap();
        assert_eq!(
            ret.similarities,
            vec![Similarity::BrandToInner, Similarity::Same].into()
        );
    }

    #[test]
    fn test_subtype_inner_to_brand() {
        let ctx = Ctx::default();
        let ret = ctx.subtype(
            &Type::Integer,
            &Type::Brand {
                brand: "a".into(),
                item: Box::new(Type::Integer),
            },
        );
        assert_eq!(
            ret,
            Err(TypeError::NotSubtype {
                sub: ty::Type::Integer.into(),
                ty: ty::Type::Brand {
                    brand: "a".into(),
                    item: Box::new(ty::Type::Integer),
                }
                .into(),
            })
        );
    }

    #[test]
    fn test_subtype_vector() {
        let ctx = Ctx::default();
        let ret = ctx
            .subtype(
                &Type::Vector(Box::new(Type::Integer)),
                &Type::Vector(Box::new(Type::Real)),
            )
            .unwrap();
        assert_eq!(
            ret.similarities,
            vec![Similarity::Vector, Similarity::Number].into()
        );
    }

    #[test]
    fn test_subtype_map() {
        let ctx = Ctx::default();
        let ret = ctx
            .subtype(
                &Type::Map {
                    key: Box::new(Type::Integer),
                    value: Box::new(Type::Integer),
                },
                &Type::Map {
                    key: Box::new(Type::Integer),
                    value: Box::new(Type::Real),
                },
            )
            .unwrap();
        assert_eq!(
            ret.similarities,
            vec![Similarity::Map, Similarity::Same].into()
        );
    }

    #[test]
    fn test_subtype_function() {
        let ctx = Ctx::default();
        let ret = ctx
            .subtype(
                &Type::Function {
                    parameter: Box::new(Type::Real),
                    body: Box::new(Type::Integer),
                },
                &Type::Function {
                    parameter: Box::new(Type::Integer),
                    body: Box::new(Type::Real),
                },
            )
            .unwrap();
        assert_eq!(
            ret.similarities,
            vec![Similarity::Function, Similarity::Number].into()
        );
    }

    #[test]
    fn test_subtype_function_parameter_not_subtype() {
        let ctx = Ctx::default();
        let ret = ctx.subtype(
            &Type::Function {
                parameter: Box::new(Type::Integer),
                body: Box::new(Type::Integer),
            },
            &Type::Function {
                parameter: Box::new(Type::Real),
                body: Box::new(Type::Integer),
            },
        );
        assert_eq!(
            ret,
            Err(TypeError::NotSubtype {
                sub: ty::Type::Real.into(),
                ty: ty::Type::Integer.into(),
            })
        );
    }

    #[test]
    fn test_subtype_function_body_not_subtype() {
        let ctx = Ctx::default();
        let ret = ctx.subtype(
            &Type::Function {
                parameter: Box::new(Type::Integer),
                body: Box::new(Type::Real),
            },
            &Type::Function {
                parameter: Box::new(Type::Integer),
                body: Box::new(Type::Integer),
            },
        );
        assert_eq!(
            ret,
            Err(TypeError::NotSubtype {
                sub: ty::Type::Real.into(),
                ty: ty::Type::Integer.into(),
            })
        );
    }

    #[test]
    fn test_subtype_instantiate_subtype() {
        let ctx = Ctx::default().add(Log::Existential(1));
        let ret = ctx.subtype(&Type::Existential(1), &Type::Integer).unwrap();
        assert_eq!(ret.similarities, vec![Similarity::Instantiate].into());
        let ret = ctx.subtype(&Type::Integer, &Type::Existential(1)).unwrap();
        assert_eq!(ret.similarities, vec![Similarity::Instantiate].into());
    }
}
