use std::collections::HashMap;

use conc_types::ConcType;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct EnumDef {
    pub variants: HashMap<ConcType, usize>,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct EnumDefs {
    pub defs: HashMap<ConcType, EnumDef>,
}

impl EnumDefs {
    pub fn get_enum_def(&mut self, ty: ConcType) -> &mut EnumDef {
        self.defs.entry(ty).or_insert_with(Default::default)
    }
}

impl EnumDef {
    pub fn get_variant_index(&mut self, ty: ConcType) -> usize {
        let id = self.variants.len();
        *self.variants.entry(ty).or_insert(id)
    }
}
