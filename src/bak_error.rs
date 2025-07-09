use thiserror::Error;

#[derive(Error, Debug)]
pub enum BakuninError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Model Error: {0}")]
    ModelError(#[from] crate::model::ModelError),
    #[error("Config Layer Error: {0}")]
    ConfigLayerError(#[from] crate::config_layer::ConfigLayerError),
    #[error("File Finder Error: {0}")]
    FileFinderError(#[from] crate::file_finder::FileFinderError),
}

pub type Result<T> = std::result::Result<T, BakuninError>;
