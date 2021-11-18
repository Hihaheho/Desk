#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Handler {
    pub input: Type,
    pub output: Type,
}

pub type Id = usize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Number,
    String,
    Product(Vec<Self>),
    Sum(Vec<Self>),
    Function {
        parameter: Box<Self>,
        body: Box<Self>,
    },
    Array(Box<Self>),
    Set(Box<Self>),
    Variable(Id),
    ForAll {
        variable: Id,
        body: Box<Self>,
    },
    Existential(Id),
}

mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
