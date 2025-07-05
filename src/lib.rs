#![doc = include_str!("../README.MD")]

#[deprecated(since = "0.4.0", note = "use `BakuninConfig` instead")]
pub mod builder;

pub mod config_layer;
pub mod file_finder;
pub mod model;

pub mod errors;
pub mod traits;

pub mod config;

pub mod config_macro;

#[allow(deprecated)]
pub use builder::{ConfigBuilder, Priority};
pub use errors::ConfigError;

pub use config::BakuninConfig;
pub use model::Value;
