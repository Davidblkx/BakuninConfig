//! Contains logic to find files in the filesystem.
//! Allows searching for files by name and extension in specified directories.
use std::path::PathBuf;

/// Allows searching for files by name and extension in specified directories.
pub struct FileFinder {
    pub(crate) folders: Vec<PathBuf>,
    pub(crate) file_name: &'static str,
    pub(crate) extensions: Vec<&'static str>,
}

impl FileFinder {
    /// Creates a new `FileFinder` instance with the specified file name.
    ///
    /// Will search for files in /[path]/[filename][extension] format.
    pub fn new(name: &'static str) -> Self {
        Self {
            file_name: name,
            extensions: Vec::new(),
            folders: Vec::new(),
        }
    }

    pub fn get_file_name(&self) -> &'static str {
        self.file_name
    }

    /// Adds an extension to the list of file extensions to search for.
    /// It must NOT include the leading dot (e.g., "json", "toml").
    pub fn with_extension(mut self, ext: &'static str) -> Self {
        self.extensions.push(ext);
        self
    }

    /// Adds a folder to the list of directories to search in.
    pub fn with_folder(mut self, folder: PathBuf) -> Self {
        self.folders.push(folder);
        self
    }

    pub fn validate(&self) -> super::Result<()> {
        if self.folders.is_empty() {
            return Err(super::FileFinderError::NoFoldersDefined);
        }
        if self.extensions.is_empty() {
            return Err(super::FileFinderError::NoExtensionsDefined);
        }
        Ok(())
    }
}
