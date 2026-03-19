use std::fs;
use std::io;
use std::path::Path;

use super::node::{FileNode, NodeType};

/// Reads a directory and returns a sorted list of [`FileNode`].
///
/// Directories come first, then entries are sorted case-insensitively by name.
/// Hidden entries (dot-files) are included but flagged via [`FileNode::is_hidden`].
pub fn list_directory(path: &Path) -> io::Result<Vec<FileNode>> {
    let mut nodes = fs::read_dir(path)?
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            let metadata = entry.metadata().ok()?;

            Some(FileNode {
                node_type: FileNode::determine_type(&path, &metadata),
                is_hidden: name.starts_with('.'),
                size: metadata.len(),
                modified: metadata.modified().ok(),
                name,
                path,
            })
        })
        .collect::<Vec<_>>();

    // Directories first, then alphabetical (case-insensitive).
    nodes.sort_by_key(|n| (n.node_type != NodeType::Directory, n.name.to_lowercase()));

    Ok(nodes)
}