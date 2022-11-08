use std::{collections::HashSet, sync::Arc};

use deskc_ids::NodeId;

#[salsa::query_group(KernelStorage)]
pub trait DescendantsQueries {
    #[salsa::input]
    fn node(&self, id: NodeId) -> Arc<HashSet<NodeId>>;
    fn descendants(&self, id: NodeId) -> Arc<HashSet<NodeId>>;
    fn does_make_loop(&self, start: NodeId, end: NodeId) -> bool;
}

#[salsa::database(KernelStorage)]
#[derive(Default)]
pub struct Descendants {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for Descendants {}

fn descendants(db: &dyn DescendantsQueries, node_id: NodeId) -> Arc<HashSet<NodeId>> {
    let mut ret = db.node(node_id).as_ref().clone();
    let mut node_ids: Vec<NodeId> = ret.iter().cloned().collect();
    let mut next_node_ids;
    while !node_ids.is_empty() {
        next_node_ids = Vec::new();
        for node_id in node_ids {
            let children = db.descendants(node_id.clone());
            next_node_ids.extend(children.difference(&ret).cloned());
            ret = ret.union(&children).cloned().collect();
        }
        node_ids = next_node_ids.to_vec();
    }
    Arc::new(ret)
}

fn does_make_loop(db: &dyn DescendantsQueries, start: NodeId, end: NodeId) -> bool {
    db.descendants(end).contains(&start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_descendants() {
        // a - b - c
        //   + d - e
        let node_a = NodeId::new();
        let node_b = NodeId::new();
        let node_c = NodeId::new();
        let node_d = NodeId::new();
        let node_e = NodeId::new();
        let mut detector = Descendants::default();
        detector.set_node(
            node_a.clone(),
            Arc::new(vec![node_b.clone(), node_d.clone()].into_iter().collect()),
        );
        detector.set_node(
            node_b.clone(),
            Arc::new(vec![node_c.clone()].into_iter().collect()),
        );
        detector.set_node(node_c.clone(), Arc::new(vec![].into_iter().collect()));
        detector.set_node(
            node_d.clone(),
            Arc::new(vec![node_e.clone()].into_iter().collect()),
        );
        detector.set_node(node_e.clone(), Arc::new(vec![].into_iter().collect()));
        assert_eq!(
            detector.descendants(node_a),
            Arc::new(
                [node_b.clone(), node_c.clone(), node_d, node_e]
                    .into_iter()
                    .collect()
            )
        );
        assert_eq!(
            detector.descendants(node_b),
            Arc::new([node_c.clone()].into_iter().collect())
        );
        assert_eq!(
            detector.descendants(node_c),
            Arc::new([].into_iter().collect())
        );
    }
}
