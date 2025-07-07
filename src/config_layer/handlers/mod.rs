#[cfg(feature = "toml")]
mod toml;

#[cfg(feature = "json")]
mod json;

#[cfg(feature = "toml")]
pub use toml::TomlFileHandler;

#[cfg(feature = "json")]
pub use json::JsonFileHandler;