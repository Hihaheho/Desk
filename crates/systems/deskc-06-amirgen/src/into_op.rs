use amir::stmt::Op;

pub(crate) fn into_op(op: &thir::BuiltinOp) -> Op {
    match op {
        thir::BuiltinOp::Add => Op::Add,
        thir::BuiltinOp::Sub => Op::Sub,
        thir::BuiltinOp::Mul => Op::Mul,
        thir::BuiltinOp::Div => Op::Div,
        thir::BuiltinOp::Rem => Op::Rem,
        thir::BuiltinOp::Mod => Op::Mod,
        thir::BuiltinOp::Eq => Op::Eq,
        thir::BuiltinOp::Neq => Op::Neq,
        thir::BuiltinOp::Lt => Op::Lt,
        thir::BuiltinOp::Le => Op::Le,
        thir::BuiltinOp::Gt => Op::Gt,
        thir::BuiltinOp::Ge => Op::Ge,
        thir::BuiltinOp::And => panic!(),
        thir::BuiltinOp::Or => panic!(),
        thir::BuiltinOp::Not => Op::Not,
        thir::BuiltinOp::Neg => Op::Neg,
        thir::BuiltinOp::BitAnd => Op::BitAnd,
        thir::BuiltinOp::BitOr => Op::BitOr,
        thir::BuiltinOp::BitXor => Op::BitXor,
        thir::BuiltinOp::BitNot => Op::BitNot,
        thir::BuiltinOp::Shl => Op::Shl,
        thir::BuiltinOp::Shr => Op::Shr,
    }
}
