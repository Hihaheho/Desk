use std::collections::HashMap;

use amir::{
    amir::{Amir, AmirId},
    block::{ABasicBlock, BlockId},
    link::{ALink, LinkId},
    scope::ScopeId,
    stmt::{AStmt, ATerminator, StmtBind},
    var::{AVar, VarId},
};
use types::{Effect, Type};

use crate::{block_proto::BlockProto, scope_proto::ScopeProto};

pub struct AmirProto {
    parameters: Vec<Type>,
    scopes: Vec<ScopeProto>,
    blocks_proto: HashMap<BlockId, BlockProto>,
    blocks: HashMap<BlockId, ABasicBlock>,
    vars: Vec<AVar>,
    links: Vec<ALink>,
    links_map: HashMap<Type, LinkId>,
    // Scope stack
    current_scope: Vec<ScopeId>,
    // Block stack
    current_block: Vec<BlockId>,
    // Values that referenced but not included in parameter
    captured_values: HashMap<Type, VarId>,
    handlers: HashMap<Effect, AmirId>,
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
            blocks_proto: [(BlockId(0), BlockProto::default())].into_iter().collect(),
            blocks: HashMap::default(),
            captured_values: HashMap::default(),
            handlers: HashMap::default(),
        }
    }
}

impl AmirProto {
    pub fn into_amir(self, var: VarId, output: Type) -> Amir {
        let current_block_id = self.current_block_id();
        let AmirProto {
            parameters,
            scopes,
            mut blocks,
            mut blocks_proto,
            vars,
            links,
            captured_values,
            ..
        } = self;
        // Save the last block
        assert!(blocks_proto.len() == 1);
        let last_block = blocks_proto.remove(&current_block_id).unwrap();
        blocks.insert(
            current_block_id,
            ABasicBlock {
                stmts: last_block.stmts,
                terminator: ATerminator::Return(var),
            },
        );

        // Sort the blocks by block_id
        let mut blocks: Vec<_> = blocks.into_iter().collect();
        blocks.sort_by(|a, b| a.0 .0.partial_cmp(&b.0 .0).unwrap());
        // Block Id must be exhausive
        assert_eq!(blocks[0].0, BlockId(0));
        let last_block_id = blocks.len() - 1;
        assert_eq!(blocks[last_block_id].0, BlockId(last_block_id));
        let blocks = blocks.into_iter().map(|(_, b)| b).collect();

        // Handle captured variables, add to parameters
        let captured = captured_values
            .into_iter()
            .map(|(ty, _var_id)| ty)
            .collect();

        Amir {
            parameters,
            captured,
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
        self.get_scope(&self.current_scope_id())
    }

    pub fn get_scope(&mut self, id: &ScopeId) -> &mut ScopeProto {
        self.scopes.get_mut(id.0).unwrap()
    }

    pub fn current_block_id(&self) -> BlockId {
        self.current_block.last().copied().unwrap()
    }

    pub fn block_proto(&mut self) -> &mut BlockProto {
        self.blocks_proto.get_mut(&self.current_block_id()).unwrap()
    }

    pub fn find_var(&mut self, ty: &Type) -> VarId {
        let mut next_scope_id = Some(self.current_scope_id());
        while let Some(scope_id) = next_scope_id {
            let scope = self.get_scope(&scope_id);
            next_scope_id = scope.super_id;
            if let Some(var_id) = scope.named_vars.get(ty) {
                return *var_id;
            }
        }
        self.captured_values.get(ty).cloned().unwrap_or_else(|| {
            let var = self.create_var(ty.clone());
            self.bind_to(var, AStmt::Parameter);
            var
        })
    }

    pub fn create_var(&mut self, ty: Type) -> VarId {
        let id = VarId(self.vars.len());
        self.vars.push(AVar {
            scope: self.current_scope_id(),
            ty,
        });
        id
    }

    pub fn create_named_var(&mut self, var: VarId) {
        let ty = self.get_var(&var).ty.clone();
        self.current_scope().named_vars.insert(ty, var);
    }

    pub fn get_var(&mut self, var_id: &VarId) -> &mut AVar {
        &mut self.vars[var_id.0]
    }

    pub fn bind_stmt(&mut self, ty: Type, stmt: AStmt) -> VarId {
        let var = self.create_var(ty);
        self.bind_to(var, stmt)
    }

    pub(crate) fn bind_to(&mut self, var: VarId, stmt: AStmt) -> VarId {
        let stmt_bind = StmtBind { var, stmt };
        self.block_proto().push_stmt_bind(stmt_bind);
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

    pub fn end_scope_then_return<T>(&mut self, var: T) -> T {
        self.current_scope.pop();
        var
    }

    pub fn begin_block(&mut self) -> BlockId {
        let id = BlockId(self.blocks.len() + self.blocks_proto.len());
        self.current_block.push(id);
        self.blocks_proto.insert(id, BlockProto::default());
        id
    }

    // begin block handled the next time of current block
    pub fn begin_block_defer(&mut self) -> BlockId {
        let id = self.begin_block();
        let len = self.current_block.len();
        // defer
        self.current_block.swap(len - 1, len - 2);
        id
    }

    pub fn end_block(&mut self, terminator: ATerminator) -> BlockId {
        let id = self.current_block.pop().unwrap();
        let stmts = self.blocks_proto.remove(&id).unwrap().stmts;
        self.blocks.insert(id, ABasicBlock { stmts, terminator });
        id
    }

    pub fn request_link(&mut self, ty: Type) -> LinkId {
        let id = LinkId(self.links.len());
        self.links.push(ALink { ty: ty.clone() });
        self.links_map.entry(ty).or_insert(id).clone()
    }

    pub(crate) fn assign_handler(&mut self, effect: Effect, handler_amir: amir::amir::AmirId) {
        self.handlers.insert(effect, handler_amir);
    }
}
