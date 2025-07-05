//! This module defines the `OSDirectory` enum, which represents different directories in the operating system.

use std::path::PathBuf;

#[derive(Debug)]
/// This enum represents various directories in the operating system.
pub enum OSDirectory {
    /// The current working directory
    WorkingDirectory,
    /// Home directory of the user
    ///
    /// Windows: {FOLDERID_Profile} -> C:\Users\{username} <br />
    /// Linux: $HOME -> /home/{username} <br />
    /// macOS: $HOME -> /Users/{username} <br />
    UserHome,
    /// Configuration directory of the user
    ///
    /// Windows: {FOLDERID_RoamingAppData} -> C:\Users\{username}\AppData\Roaming <br />
    /// Linux: $XDG_CONFIG_HOME or $HOME/.config -> /home/{username}/.config <br />
    /// macOS: $HOME/Library/Application Support -> /Users/{username}/Library/Application Support <br />
    UserConfig,
    /// Preferences directory of the user
    ///
    /// Windows: {FOLDERID_RoamingAppData} -> C:\Users\{username}\AppData\Roaming <br />
    /// Linux: $XDG_CONFIG_HOME or $HOME/.config -> /home/{username}/.config <br />
    /// macOS: $HOME/Library/Preferences -> /Users/{username}/Library/Preferences <br />
    UserPreferences,
    /// Application config directory
    ///
    /// Windows: {FOLDERID_RoamingAppData}\{AppName}\config -> C:\Users\{username}\AppData\Roaming\{AppName}\config <br />
    /// Linux: $XDG_CONFIG_HOME or $HOME/.config/{AppName} -> /home/{username}/.config/{AppName} <br />
    /// macOS: $HOME/Library/Application Support/{AppName} -> /Users/{username}/Library/Application Support/{AppName} <br />
    AppConfig(&'static str),
    /// Application home directory
    ///
    /// Windows: {FOLDERID_Profile}\{AppName} -> C:\Users\{username}\{AppName} <br />
    /// Linux: $HOME/{AppName} -> /home/{username}/{AppName} <br />
    /// macOS: $HOME/{AppName} -> /Users/{username}/{AppName} <br />
    AppHome(&'static str),

    /// Custom path specified by the user
    Path(String),
}

impl OSDirectory {
    /// Converts the `OSDirectory` to a `PathBuf`.
    pub fn to_path_buf(&self) -> Option<std::path::PathBuf> {
        if let Some(dir) = directories::BaseDirs::new() {
            return match self {
                OSDirectory::UserHome => Some(dir.home_dir().to_path_buf()),
                OSDirectory::UserConfig => Some(dir.config_dir().to_path_buf()),
                OSDirectory::UserPreferences => Some(dir.preference_dir().to_path_buf()),
                OSDirectory::WorkingDirectory => std::env::current_dir().ok(),
                OSDirectory::AppHome(app_name) => Some(dir.home_dir().to_path_buf().join(app_name)),
                OSDirectory::AppConfig(app_name) => {
                    Some(dir.config_dir().to_path_buf().join(app_name))
                },
                OSDirectory::Path(path) => Some(PathBuf::from(path)),
            };
        }

        None
    }
}

impl super::FileFinder {
    /// Adds a folder to the list of directories to search in, based on the `OSDirectory`.
    pub fn with_os_directory(mut self, os_dir: OSDirectory) -> Self {
        if let Some(path) = os_dir.to_path_buf() {
            self.folders.push(path);
        } else {
            log::warn!("Failed to resolve OS directory: {:?}", os_dir);
        }
        self
    }

    /// Adds the current working directory to the list of directories to search in.
    pub fn with_working_directory(self) -> Self {
        self.with_os_directory(OSDirectory::WorkingDirectory)
    }

    /// Adds the user's home directory to the list of directories to search in.
    pub fn with_user_home(self) -> Self {
        self.with_os_directory(OSDirectory::UserHome)
    }

    /// Adds the user's configuration directory to the list of directories to search in.
    pub fn with_user_config(self) -> Self {
        self.with_os_directory(OSDirectory::UserConfig)
    }

    /// Adds the user's preferences directory to the list of directories to search in.
    pub fn with_user_preferences(self) -> Self {
        self.with_os_directory(OSDirectory::UserPreferences)
    }

    /// Adds the application's configuration directory to the list of directories to search in.
    pub fn with_app_config(self, app_name: &'static str) -> Self {
        self.with_os_directory(OSDirectory::AppConfig(app_name))
    }

    /// Adds the application's home directory to the list of directories to search in.
    pub fn with_app_home(self, app_name: &'static str) -> Self {
        self.with_os_directory(OSDirectory::AppHome(app_name))
    }

    /// Adds a custom path to the list of directories to search in.
    pub fn with_path(self, path: String) -> Self {
        self.with_os_directory(OSDirectory::Path(path))
    }
}
