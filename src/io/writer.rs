//! The disk boundary. The runner depends on the `FileSystem` abstraction;
//! `main` injects `RealFileSystem`, tests inject `InMemoryFs`.

use crate::error::CliError;
use std::path::Path;

pub trait FileSystem {
    fn exists(&self, path: &Path) -> bool;
    fn create_dir_all(&self, path: &Path) -> Result<(), CliError>;
    fn write(&self, path: &Path, contents: &str) -> Result<(), CliError>;
}

/// Production sink: the real filesystem.
pub struct RealFileSystem;

impl FileSystem for RealFileSystem {
    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn create_dir_all(&self, path: &Path) -> Result<(), CliError> {
        std::fs::create_dir_all(path).map_err(|source| CliError::Io {
            path: path.display().to_string(),
            source,
        })
    }

    fn write(&self, path: &Path, contents: &str) -> Result<(), CliError> {
        std::fs::write(path, contents).map_err(|source| CliError::Io {
            path: path.display().to_string(),
            source,
        })
    }
}

/// Test sink: keeps written files in memory.
#[cfg(test)]
pub struct InMemoryFs {
    pub files: std::cell::RefCell<std::collections::HashMap<std::path::PathBuf, String>>,
    pub dirs: std::cell::RefCell<std::collections::HashSet<std::path::PathBuf>>,
}

#[cfg(test)]
impl Default for InMemoryFs {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
impl InMemoryFs {
    pub fn new() -> Self {
        Self {
            files: std::cell::RefCell::new(std::collections::HashMap::new()),
            dirs: std::cell::RefCell::new(std::collections::HashSet::new()),
        }
    }

    /// Seed a pre-existing file (to exercise collision handling).
    pub fn seed(&self, path: &Path, contents: &str) {
        self.files
            .borrow_mut()
            .insert(path.to_path_buf(), contents.to_string());
    }
}

#[cfg(test)]
impl FileSystem for InMemoryFs {
    fn exists(&self, path: &Path) -> bool {
        self.files.borrow().contains_key(path)
    }

    fn create_dir_all(&self, path: &Path) -> Result<(), CliError> {
        self.dirs.borrow_mut().insert(path.to_path_buf());
        Ok(())
    }

    fn write(&self, path: &Path, contents: &str) -> Result<(), CliError> {
        self.files
            .borrow_mut()
            .insert(path.to_path_buf(), contents.to_string());
        Ok(())
    }
}
