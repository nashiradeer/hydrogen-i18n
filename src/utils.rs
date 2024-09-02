//! # Hydrogen I18n // Utils
//!
//! Utilities for the Hydrogen I18n library.

use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

/// Search for files in the given path.
pub fn search_files<A: AsRef<Path>>(path: A) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut dirs = vec![path.as_ref().to_path_buf()];

    while let Some(dir) = dirs.pop() {
        if let Ok(entries) = read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    dirs.push(path);
                } else {
                    files.push(path);
                }
            }
        }
    }

    files
}

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
/// Search for files in the given path asynchronously using [`tokio`].
pub async fn tokio_search_files<A: AsRef<Path>>(path: A) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut dirs = vec![path.as_ref().to_path_buf()];

    while let Some(dir) = dirs.pop() {
        if let Ok(mut entries) = tokio::fs::read_dir(dir).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if path.is_dir() {
                    dirs.push(path);
                } else {
                    files.push(path);
                }
            }
        }
    }

    files
}
