use mir::{stmt::Stmt, StmtBind, VarId};

use super::BlockConcretizer;

impl<'a> BlockConcretizer<'a> {
    pub fn cast_to(&mut self, to_var_id: VarId, from_var_id: VarId) {
        let from = &self.vars.get(&from_var_id);
        let to = &self.vars.get(&to_var_id);
        match (&from.ty, &to.ty) {
            (x, y) if x == y => self.stmts.push(StmtBind {
                stmt: Stmt::Move(from_var_id),
                var: to_var_id,
            }),
            (mir::ty::ConcType::Tuple(types), inner) if types.contains(inner) => todo!("index"),
            (inner, mir::ty::ConcType::Enum(types)) if types.contains(inner) => {
                let variant_id = self
                    .enum_defs
                    .get_enum_def(to.ty.clone())
                    .get_variant_index(from.ty.clone());
                self.stmts.push(StmtBind {
                    var: to_var_id,
                    stmt: Stmt::Variant {
                        id: variant_id,
                        value: from_var_id,
                    },
                });
            }
            _ => panic!("cannot cast {:?} to {:?}", from, to.ty),
        }
    }
}
