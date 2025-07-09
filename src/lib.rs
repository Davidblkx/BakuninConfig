#![doc = include_str!("../README.MD")]

pub mod config_layer;
pub mod file_finder;
pub mod model;

pub mod bak_error;

pub mod config;

pub mod config_macro;

pub use bak_error::{BakuninError, Result};
pub use config::BakuninConfig;
pub use model::Value;
