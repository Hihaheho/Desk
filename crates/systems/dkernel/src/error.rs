use deskc_ids::NodeId;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum KernelError {
    #[error("{node_id:?}")]
    NoEntrypoint { node_id: NodeId },
}
