use std::collections::HashMap;

use amir::{
    amir::Amir,
    block::{BasicBlock, BlockId},
    link::{Link, LinkId},
    scope::ScopeId,
    stmt::{Stmt, StmtBind, Terminator},
    var::{Var, VarId},
};
use types::Type;

use crate::{block_proto::BlockProto, scope_proto::ScopeProto};

pub struct AmirProto {
    parameters: Vec<Type>,
    scopes: Vec<ScopeProto>,
    blocks_proto: Vec<BlockProto>,
    blocks: Vec<BasicBlock>,
    vars: Vec<Var>,
    links: Vec<Link>,
    links_map: HashMap<Type, LinkId>,
    current_scope: Vec<ScopeId>,
    current_block: Vec<BlockId>,
}

impl Default for AmirProto {
    fn default() -> Self {
        Self {
            parameters: vec![],
            current_scope: vec![ScopeId(0)],
            current_block: vec![BlockId(0)],
            vars: vec![],
            links: vec![],
            links_map: HashMap::default(),
            scopes: vec![ScopeProto::default()],
            blocks_proto: vec![BlockProto::default()],
            blocks: vec![],
        }
    }
}

impl AmirProto {
    pub fn into_amir(self, output: Type) -> Amir {
        let AmirProto {
            parameters,
            scopes,
            mut blocks,
            mut blocks_proto,
            vars,
            links,
            ..
        } = self;
        assert!(blocks_proto.len() == 1);
        let last_block = blocks_proto.pop().unwrap();
        // If last block have stmt.
        if last_block.stmts.len() != 0 {
            let ret = last_block.stmts.last().as_ref().unwrap().var;
            blocks.push(BasicBlock {
                stmts: last_block.stmts,
                terminator: Terminator::Return(ret),
            })
        }
        Amir {
            parameters,
            output,
            vars,
            scopes: scopes.into_iter().map(|s| s.into_scope()).collect(),
            blocks,
            links,
        }
    }

    pub fn current_scope_id(&self) -> ScopeId {
        self.current_scope.last().copied().unwrap()
    }

    pub fn current_scope(&mut self) -> &mut ScopeProto {
        let id = self.current_scope_id().0;
        self.scopes.get_mut(id).unwrap()
    }

    pub fn block(&mut self) -> &mut BlockProto {
        self.blocks_proto
            .get_mut(self.current_block.last().unwrap().0)
            .unwrap()
    }

    pub fn find_var(&self, ty: &Type) -> Option<VarId> {
        self.scopes
            .iter()
            .take(1 + self.current_scope_id().0)
            .rev()
            .find_map(|scope| scope.named_vars.get(ty))
            .cloned()
    }

    pub fn create_var(&mut self, ty: Type) -> VarId {
        let id = VarId(self.vars.len());
        self.vars.push(Var {
            scope: self.current_scope_id(),
            ty,
        });
        id
    }

    pub fn create_named_var(&mut self, var: VarId, ty: Type) {
        self.current_scope().named_vars.insert(ty, var);
    }

    pub fn bind_stmt(&mut self, ty: Type, stmt: Stmt) -> VarId {
        let var = self.create_var(ty);
        let stmt_bind = StmtBind { var, stmt };
        self.block().push_stmt_bind(stmt_bind);
        var
    }

    pub fn begin_scope(&mut self) {
        let super_scope_id = self.current_scope_id();
        let id = ScopeId(self.scopes.len());
        // Create new scope
        self.current_scope.push(id);
        self.scopes.push(ScopeProto {
            super_id: Some(super_scope_id),
            ..Default::default()
        });
    }

    pub fn end_scope<T>(&mut self, var: T) -> T {
        self.current_scope.pop();
        var
    }

    pub fn request_link(&mut self, ty: Type) -> LinkId {
        let id = LinkId(self.links.len());
        self.links.push(Link { ty: ty.clone() });
        self.links_map.entry(ty).or_insert(id).clone()
    }
}
