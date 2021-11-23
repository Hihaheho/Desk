use amir::{
    amir::Amir,
    block::ABasicBlock,
    link::ALink,
    stmt::{AStmt, StmtBind, Terminator},
    var::AVar,
};
use mir::{
    mir::{BasicBlock, Link, Mir},
    stmt::Stmt,
    ty::{ConcEffect, ConcType},
};
use types::{Effect, Type};

pub struct MirGenOptions {
    pub parameters: Vec<ConcType>,
    pub output: ConcType,
}

pub fn gen_mir(options: MirGenOptions, amir: &Amir) -> Mir {
    let Amir {
        parameters: _,
        output: _,
        vars,
        scopes,
        blocks,
        links,
    } = amir;

    // This is temporary implementation.

    let vars = vars
        .iter()
        .map(|var| AVar {
            ty: to_conc_type(&var.ty),
            scope: var.scope,
        })
        .collect();

    let blocks = blocks
        .iter()
        .map(|ABasicBlock { stmts, terminator }| {
            let stmts = stmts
                .iter()
                .map(|StmtBind { stmt, var }| {
                    let stmt = match stmt {
                        AStmt::Const(value) => Stmt::Const(value.clone()),
                        AStmt::Product(values) => Stmt::Product(values.iter().cloned().collect()),
                        AStmt::Array(values) => Stmt::Array(values.iter().cloned().collect()),
                        AStmt::Set(values) => Stmt::Set(values.iter().cloned().collect()),
                        AStmt::Fn(fn_ref) => Stmt::Fn(*fn_ref),
                        AStmt::Perform(var) => Stmt::Perform(*var),
                        AStmt::Apply {
                            function,
                            arguments,
                        } => todo!(),
                        AStmt::Op { op, operands } => Stmt::Op {
                            op: op.clone(),
                            operands: operands.clone(),
                        },
                    };
                    StmtBind { stmt, var: *var }
                })
                .collect();
            let terminator = match terminator {
                Terminator::Return(var) => Terminator::Return(*var),
                Terminator::Match { var, cases } => todo!(),
            };
            BasicBlock { stmts, terminator }
        })
        .collect();

    let links = links
        .iter()
        .map(|ALink { ty }| Link {
            ty: to_conc_type(ty),
        })
        .collect();

    Mir {
        parameters: options.parameters.clone(),
        output: options.output,
        vars,
        scopes: scopes.clone(),
        blocks,
        links,
    }
}

fn to_conc_type(ty: &Type) -> ConcType {
    match ty {
        Type::Number => ConcType::Number,
        Type::String => ConcType::String,
        Type::Product(types) => ConcType::Product(types.iter().map(|t| to_conc_type(t)).collect()),
        Type::Sum(types) => ConcType::Sum(types.iter().map(|t| to_conc_type(t)).collect()),
        Type::Function { parameters, body } => ConcType::Function {
            parameters: parameters.iter().map(|t| to_conc_type(t)).collect(),
            body: Box::new(to_conc_type(body)),
        },
        Type::Array(ty) => ConcType::Array(Box::new(to_conc_type(ty))),
        Type::Set(ty) => ConcType::Set(Box::new(to_conc_type(ty))),
        Type::Variable(id) => ConcType::Variable(id.clone()),
        Type::ForAll { variable, body } => ConcType::ForAll {
            variable: variable.clone(),
            body: Box::new(to_conc_type(body)),
        },
        Type::Effectful { ty, effects } => ConcType::Effectful {
            ty: Box::new(to_conc_type(ty)),
            effects: effects
                .iter()
                .map(|Effect { input, output }| ConcEffect {
                    input: to_conc_type(input),
                    output: to_conc_type(output),
                })
                .collect(),
        },
        Type::Brand { brand, item } => todo!(),
        Type::Label { label, item } => todo!(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
