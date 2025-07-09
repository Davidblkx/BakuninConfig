use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigLayerError {
    #[error("Writing to this layer is not supported")]
    WriteNotSupported,

    #[error("Failed to read value: {0}")]
    ErrorReadingValue(String),

    #[error("Failed to write value: {0}")]
    ErrorWritingValue(String),

    #[error("Failed to parse value: {0}")]
    ParseError(String),

    #[error("Failed to serialize value: {0}")]
    SerializeError(String),

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Unsupported extension: {0}")]
    UnsupportedExtension(String),
}
