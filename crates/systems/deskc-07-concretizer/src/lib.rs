mod block_concretizer;
mod enumdef;
mod type_concretizer;

use amir::{
    amir::{Amir, ControlFlowGraph as ACFG},
    var::AVar,
};
use block_concretizer::BlockConcretizer;
use conc_types::ConcType;
use enumdef::EnumDefs;
use mir::{
    mir::{ControlFlowGraph as CFG, LinkId, Mir},
    Vars,
};
use type_concretizer::TypeConcretizer;

pub struct Concretizer {
    pub parameters: Vec<ConcType>,
    pub output: ConcType,
    pub enum_defs: EnumDefs,
}

pub fn concretize(amirs: &Amir) -> Mir {
    let mirs = amirs
        .cfgs
        .iter()
        .map(|amir| {
            let mut concretizer = Concretizer {
                parameters: vec![],
                output: ConcType::Number,
                enum_defs: Default::default(),
            };
            concretizer.concretize_mir(amir)
        })
        .collect();
    Mir {
        entrypoint: amirs.entrypoint,
        cfgs: mirs,
    }
}

impl Concretizer {
    fn concretize_mir(&mut self, amir: &ACFG) -> CFG {
        let ACFG {
            parameters: _,
            output: _,
            vars,
            scopes,
            blocks,
            captured,
            links,
        } = amir;
        let mut type_conc = TypeConcretizer {};

        let mut vars = Vars(
            vars.iter()
                .map(|var| AVar {
                    ty: type_conc.gen_conc_type(&var.ty),
                    scope: var.scope,
                })
                .collect(),
        );

        let mut block_conc = BlockConcretizer {
            enum_defs: &mut self.enum_defs,
            type_concretizer: &mut type_conc,
            vars: &mut vars,
            stmts: vec![],
        };
        let blocks = block_conc.concretize_blocks(blocks);

        let captured = captured
            .iter()
            .map(|ty| type_conc.gen_conc_type(ty))
            .collect();

        let links = links
            .iter()
            .map(|link| LinkId {
                ty: type_conc.gen_conc_type(&link.ty),
                name: link.name.clone(),
            })
            .collect();

        CFG {
            parameters: self.parameters.clone(),
            captured,
            output: self.output.clone(),
            vars,
            scopes: scopes.clone(),
            blocks,
            links,
        }
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
