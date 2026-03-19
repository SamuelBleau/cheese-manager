use std::path::PathBuf;
use std::time::SystemTime;

/// The type of the file system node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    File,
    Directory,
    Symlink,
    Archive,
    Unknown,
}

/// Represents an item in the file system (file, directory, symlink, archive, etc.)
#[derive(Debug, Clone)]
pub struct FileNode {
    pub name: String,
    pub path: PathBuf,
    pub node_type: NodeType,
    pub size: u64,
    pub modified: Option<SystemTime>,
    pub is_hidden: bool,
}

impl FileNode {
    /// Determines the [`NodeType`] from standard [`std::fs::Metadata`].
    pub fn determine_type(path: &std::path::Path, metadata: &std::fs::Metadata) -> NodeType {
        if metadata.is_dir() {
            NodeType::Directory
        } else if metadata.is_symlink() {
            NodeType::Symlink
        } else if metadata.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str())
                && matches!(ext, "zip" | "tar" | "gz" | "xz")
            {
                return NodeType::Archive;
            }
            NodeType::File
        } else {
            NodeType::Unknown
        }
    }
}