use deskc_ids::FileId;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct File {
    name: Option<String>,
    children: Vec<FileId>,
    // TODO: rules
}
