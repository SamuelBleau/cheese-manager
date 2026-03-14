use std::fs;
use std::io;
use std::path::Path;

use super::node::{FileNode, NodeType};

/// Reads a directory and returns a list of FileNode
pub fn list_directory(path: &Path) -> io::Result<Vec<FileNode>> {
    let mut nodes = Vec::new();

    let entries = fs::read_dir(path)?;

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        
        // Skip hidden files if we want, or at least flag them
        let is_hidden = name.starts_with('.');

        if let Ok(metadata) = entry.metadata() {
            let node_type = FileNode::determine_type(&path, &metadata);
            
            nodes.push(FileNode {
                name,
                path,
                node_type,
                size: metadata.len(),
                modified: metadata.modified().ok(),
                is_hidden,
            });
        }
    }

    // Sort: Directories first, then alphabetical
    nodes.sort_by(|a, b| {
        let a_is_dir = a.node_type == NodeType::Directory;
        let b_is_dir = b.node_type == NodeType::Directory;
        
        if a_is_dir && !b_is_dir {
            std::cmp::Ordering::Less
        } else if !a_is_dir && b_is_dir {
            std::cmp::Ordering::Greater
        } else {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        }
    });

    Ok(nodes)
}
