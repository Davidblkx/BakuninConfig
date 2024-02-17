use std::fmt;

use thiserror::Error;
use serde::{de, ser};

pub type Result<T> = std::result::Result<T, ConfigError>;

#[derive(Error, Debug, PartialEq)]
pub enum ConfigError {
    #[error("Can't convert {value:?} to {target:?}
    ")]
    ConversionError {
        target: String,
        value: String,
    },
    #[error("Can't deserialize: {0}")]
    DeserializeError(String),
    #[error("Can't serialize: {0}")]
    SerializeError(String),
    #[error("Can't convert {from:?} to {to:?}: {why:?}")]
    CastingError {
        from: String,
        to: String,
        why: String,
    },
    #[error("Unsupported conversion from {from:?} to {to:?}")]
    UnsupportedConversion {
        from: String,
        to: String,
    },
    #[error("Operation only supported for map")]
    NotMapValue,
    #[error("Operation only supported for array")]
    NotArrayValue,
    #[error("Operation not supported for {value:?}")]
    NotSupported { value: String },

    // ------------------------------------------------------------

    #[error("Config error in source {name:?}: {error:?}")]
    Source{ name: String, error: String },

    #[error("Config error in file: {0}")]
    FileOperation(String),

    #[error("Config error in environment: {0}")]
    Environment(String),

    #[error("Error importing config source: {0}")]
    ImportingSource(String),

    // ------------------------------------------------------------
    #[error("Priority {0} is already in use")]
    BuilderPriorityInUse(u64),

    #[error("Priority 0 is reserved for the base value")]
    BuilderPriorityReserved,

    #[error("Priority not defined")]
    BuilderPriorityNotFound,
}

impl de::Error for ConfigError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::DeserializeError(msg.to_string())
    }
}

impl ser::Error for ConfigError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::SerializeError(msg.to_string())
    }
}

#[macro_export]
macro_rules! config_source_error {
    ($name:expr, $error:expr) => {
        ConfigError::Source {
            name: $name.to_string(),
            error: $error.to_string() 
        }
    };
}

#[macro_export]
macro_rules! config_file_error {
    ($error:expr) => {
        ConfigError::FileOperation($error.to_string())
    };
}

#[macro_export]
macro_rules! casting_error {
    ($from:ty, $to:ty, $why:expr) => {
        Err(ConfigError::CastingError {
            from: stringify!($from).to_string(),
            to: stringify!($to).to_string(),
            why: $why.to_string(),
        })
    };
}

#[macro_export]
macro_rules! unsupported_conversion_error {
    ($from:ty, $to:ty) => {
        Err(ConfigError::UnsupportedConversion {
            from: stringify!($from).to_string(),
            to: stringify!($to).to_string(),
        })
    };
}

#[macro_export]
macro_rules! deserialize_error {
    ($msg:expr) => {
        ConfigError::DeserializeError($msg.to_string())
    };
}

#[macro_export]
macro_rules! serialize_error {
    ($msg:expr) => {
        ConfigError::SerializeError($msg.to_string())
    };
}
