use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FileId(pub usize);

pub struct InFile<T> {
    pub id: FileId,
    pub expr: T,
}

pub enum File {
    Real(RealFile),
    Virtual(VirtualFile),
    Remote(RemoteFile),
}

pub struct RealFile {
    pub path: PathBuf,
}

pub struct VirtualFile {
    pub content: String,
}

pub struct RemoteFile {
    pub uri: String,
    pub cached_path: Option<PathBuf>,
}
