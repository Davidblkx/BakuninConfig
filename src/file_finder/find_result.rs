use std::path::PathBuf;

/// Represents the result of a file search operation.
/// Depending on the configuration, file might not exist in the filesystem,
#[derive(Debug, Clone)]
pub struct FindResult {
    /// The path to the file found.
    pub path: PathBuf,
    /// The extension of the file found.
    pub extension: super::file_extension::FileExtension,
    /// The index of the file in the search results.
    pub index: u16,
    /// Indicates whether the file exists in the filesystem.
    pub exists: bool,
}

impl FindResult {
    /// Creates a new `FindResult` instance.
    ///
    /// # Arguments
    /// * `path` - The path to the file found.
    /// * `file_name` - The name of the file.
    /// * `extension` - The extension of the file.
    pub fn new(path: PathBuf, extension: &'static str) -> Self {
        let exists = path.exists();
        Self {
            path,
            extension: super::FileExtension::from(extension),
            index: 0, // Default index, can be set later if needed
            exists,
        }
    }

    pub fn with_index(mut self, index: u16) -> Self {
        self.index = index;
        self
    }
}
