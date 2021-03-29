use card::Value;

pub type KeyId = String;
pub struct Key {
    pub id: KeyId,
    pub name: String,
    pub value_type: TableType,
}
pub struct Keys(Vec<Key>);

pub struct TableSize {
    min: Option<usize>,
    max: Option<usize>,
    exact: Option<usize>,
}

pub enum Constrain {
    Unique(Key),
}

pub struct Constrains(Vec<Constrain>);

pub struct TableType {
    keys: Keys,
    constrains: Constrains,
    record_size: TableSize,
}

impl Value for TableType {}
