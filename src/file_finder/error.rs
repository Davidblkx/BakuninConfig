use thiserror::Error;

/// Represents errors that can occur in the file finder module.
#[derive(Debug, Error)]
pub enum FileFinderError {
    /// No folders were defined.
    #[error("No folders were defined")]
    NoFoldersDefined,

    /// No files were found.
    #[error("No files were found")]
    NoFilesFound,

    /// An error occurred while reading a directory.
    #[error("IO Error: {0}")]
    ReadDirectoryError(#[from] std::io::Error),

    /// No extensions were defined.
    #[error("No extensions were defined")]
    NoExtensionsDefined,
}

pub type Result<T> = std::result::Result<T, FileFinderError>;
