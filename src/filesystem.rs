//! 
//! A small collection of file and path helpers
//! 


use serde::{Deserialize, Serialize};
use std::path::{Component, Path, PathBuf};


/// Remote file json counterpart
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoteFile {
    pub name: String,
    #[serde(rename = "isDirectory")]
    pub is_directory: bool,
    pub size: Option<u64>,
}

/// Safely joins a file name to a base path
pub fn safe_join(base: &Path, file_name: &str) -> Option<PathBuf> {
    let name_path = Path::new(file_name);
    
    let mut components = name_path.components();
    
    match (components.next(), components.next()) {
        (Some(Component::Normal(safe_name)), None) => Some(base.join(safe_name)),
        _ => None, // Rejects absolute paths, ".." traversal, empty strings, etc
    }
}