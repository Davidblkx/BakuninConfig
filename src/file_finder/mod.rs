//! This module provides functionality to find files in the filesystem.
//!
//! The `FileFinder` struct is the main entry point for finding files. It allows you to search for files in predefined OS directories or in custom directories, and it supports searching for files with specific extensions.
//!
//! Example usage:
//! ```rust
//! use bakunin_config::file_finder::FileFinder;
//!
//! let config_path = FileFinder::new(".my_app_config") // Search for files with this name
//!     .with_toml() // search for files with .toml extension
//!     .with_user_home() // Search in user directory
//!     .with_working_directory() // Search in current working directory
//!     .find_first(true) // Find the first matching file, returns the first path if none is found
//!     .unwrap().path;
//! ```

mod error;
mod file_extension;
mod find_result;
mod finder;
mod os_directory;

pub mod alg_find_all;
pub mod alg_find_all_or_first;
pub mod alg_find_first;
pub mod alg_find_last;

pub use error::FileFinderError;
pub use file_extension::FileExtension;
pub use find_result::FindResult;
pub use finder::FileFinder;
pub use os_directory::OSDirectory;
