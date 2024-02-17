pub mod builder;
pub mod file;
pub mod finder;
pub mod folders;

pub mod handler_macros;

#[cfg(feature = "json")]
pub mod handler_json;

#[cfg(feature = "toml")]
pub mod handler_toml;

#[cfg(feature = "yaml")]
pub mod handler_yaml;

#[cfg(feature = "json5")]
pub mod handler_json5;

pub use file::ConfigFile;
pub use builder::FileConfigBuilder;
pub use finder::ConfigFileFinder;
pub use folders::OSFolder;