use errors::typeinfer::TypeError;
use itertools::Itertools;

use crate::{ctx::Ctx, internal_type::Type};

impl Ctx {
    pub fn all_sum_mappings<'a>(
        &self,
        from: Vec<&'a Type>,
        to: Vec<&'a Type>,
    ) -> Result<Vec<Vec<(&'a Type, &'a Type)>>, TypeError> {
        if from.len() > to.len() {
            return Err(TypeError::SumInsufficentElements {
                sub_ty: from
                    .into_iter()
                    .map(|from| self.gen_type_or_string(from))
                    .collect(),
                super_ty: to
                    .into_iter()
                    .map(|to| self.gen_type_or_string(to))
                    .collect(),
            });
        }
        Ok(to
            .into_iter()
            .permutations(from.len())
            .map(|to| from.iter().copied().zip(to).collect())
            .collect())
    }

    // For product to product
    pub fn all_product_mappings<'a>(
        &self,
        from: Vec<&'a Type>,
        to: Vec<&'a Type>,
    ) -> Result<Vec<Vec<(&'a Type, &'a Type)>>, TypeError> {
        if from.len() < to.len() {
            return Err(TypeError::ProductInsufficentElements {
                sub_ty: from
                    .into_iter()
                    .map(|from| self.gen_type_or_string(from))
                    .collect(),
                super_ty: to
                    .into_iter()
                    .map(|to| self.gen_type_or_string(to))
                    .collect(),
            });
        }
        Ok(from
            .into_iter()
            .permutations(to.len())
            .map(|from| from.into_iter().zip(to.iter().copied()).collect())
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_sum_mappings_ok() {
        let ctx = Ctx::default();
        let from = vec![&Type::Integer, &Type::Real];
        let to = vec![&Type::Integer, &Type::Real, &Type::String];
        assert_eq!(
            ctx.all_sum_mappings(from, to),
            Ok(vec![
                vec![(&Type::Integer, &Type::Integer), (&Type::Real, &Type::Real)],
                vec![
                    (&Type::Integer, &Type::Integer),
                    (&Type::Real, &Type::String)
                ],
                vec![(&Type::Integer, &Type::Real), (&Type::Real, &Type::Integer)],
                vec![(&Type::Integer, &Type::Real), (&Type::Real, &Type::String)],
                vec![
                    (&Type::Integer, &Type::String),
                    (&Type::Real, &Type::Integer)
                ],
                vec![(&Type::Integer, &Type::String), (&Type::Real, &Type::Real)],
            ])
        );
    }

    #[test]
    fn test_all_sum_mappings_err() {
        let ctx = Ctx::default();
        let from = vec![&Type::Integer, &Type::Real];
        let to = vec![&Type::Integer];
        assert_eq!(
            ctx.all_sum_mappings(from, to),
            Err(TypeError::SumInsufficentElements {
                sub_ty: vec![ty::Type::Integer.into(), ty::Type::Real.into()],
                super_ty: vec![ty::Type::Integer.into()],
            })
        );
    }

    #[test]
    fn test_all_product_mappings_ok() {
        let ctx = Ctx::default();
        let from = vec![&Type::Integer, &Type::Real, &Type::String];
        let to = vec![&Type::Integer, &Type::Real];
        assert_eq!(
            ctx.all_product_mappings(from, to),
            Ok(vec![
                vec![(&Type::Integer, &Type::Integer), (&Type::Real, &Type::Real)],
                vec![
                    (&Type::Integer, &Type::Integer),
                    (&Type::String, &Type::Real)
                ],
                vec![(&Type::Real, &Type::Integer), (&Type::Integer, &Type::Real)],
                vec![(&Type::Real, &Type::Integer), (&Type::String, &Type::Real)],
                vec![
                    (&Type::String, &Type::Integer),
                    (&Type::Integer, &Type::Real)
                ],
                vec![(&Type::String, &Type::Integer), (&Type::Real, &Type::Real)],
            ])
        );
    }

    #[test]
    fn test_all_product_mappings_err() {
        let ctx = Ctx::default();
        let from = vec![&Type::Integer, &Type::Real];
        let to = vec![&Type::Integer, &Type::Real, &Type::String];
        assert_eq!(
            ctx.all_product_mappings(from, to),
            Err(TypeError::ProductInsufficentElements {
                sub_ty: vec![ty::Type::Integer.into(), ty::Type::Real.into()],
                super_ty: vec![
                    ty::Type::Integer.into(),
                    ty::Type::Real.into(),
                    ty::Type::String.into()
                ],
            })
        );
    }
}
