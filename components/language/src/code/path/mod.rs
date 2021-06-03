use super::node::Node;

#[derive(Clone, Debug)]
pub struct NodePath {
    pub predicates: Vec<NodePathPredicate>,
}

impl NodePath {
    pub fn new(predicates: Vec<NodePathPredicate>) -> Self {
        Self { predicates }
    }
}

#[derive(Clone, Debug)]
pub enum NodePathPredicate {}

impl Node {
    pub fn get_by_path(&self, path: &NodePath) -> &Node {
        self
    }

    pub fn patch_by_path(&self, path: &NodePath, patch: Node) -> Node {
        patch
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::code::node::sugar;

    #[test]
    fn get_by_path() {
        assert_eq!(
            sugar::integer(1).get_by_path(&NodePath { predicates: vec![] }),
            &sugar::integer(1)
        );
    }

    #[test]
    fn patch_by_path() {
        assert_eq!(
            sugar::integer(1).patch_by_path(&NodePath { predicates: vec![] }, sugar::integer(2)),
            sugar::integer(2)
        )
    }
}
