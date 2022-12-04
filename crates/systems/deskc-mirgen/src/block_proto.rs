use mir::stmt::StmtBind;

#[derive(Debug, Default)]
pub struct BlockProto {
    pub stmts: Vec<StmtBind>,
}

impl BlockProto {
    pub fn push_stmt_bind(&mut self, stmt_bind: StmtBind) {
        self.stmts.push(stmt_bind);
    }
}
