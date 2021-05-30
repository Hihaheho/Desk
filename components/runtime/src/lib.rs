use language::abstract_syntax_tree::node::Node;

pub trait Runtime {
    type Code;
    type Error;

    fn generate_code(&self, ir: &Node) -> Self::Code;
    fn run(&self, code: &Self::Code) -> Result<Node, Self::Error>;
}
