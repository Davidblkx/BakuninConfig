use directories::{BaseDirs, ProjectDirs};

/// Cross-platform folder locations
pub enum OSFolder {
    /// The current working directory
    WorkingDirectory,
    /// The user's configuration directory ex: /home/user/.config
    Config,
    /// The user's home directory ex: /home/user
    UserHome,
    /// The user's data directory ex: /home/user/.cache
    Cache,
    /// The user's application cache directory ex: /home/user/.cache/app_name
    AppCache(String),
    /// The user's application configuration directory ex: /home/user/.config/app_name
    AppConfig(String),
}

impl OSFolder {
    pub fn to_path_buf(&self) -> Option<std::path::PathBuf> {
        match self {
            OSFolder::WorkingDirectory => std::env::current_dir().ok(),
            OSFolder::Config => BaseDirs::new().map(|d| d.config_dir().to_path_buf()),
            OSFolder::UserHome => BaseDirs::new().map(|dirs| dirs.home_dir().to_path_buf()),
            OSFolder::Cache => BaseDirs::new().map(|dirs| dirs.cache_dir().to_path_buf()),
            OSFolder::AppCache(app_name) => ProjectDirs::from("com", &app_name, &app_name)
                .map(|dirs| dirs.cache_dir().to_path_buf()),
            OSFolder::AppConfig(app_name) => ProjectDirs::from("com", &app_name, &app_name)
                .map(|dirs| dirs.config_dir().to_path_buf()),
        }
    }
}

impl super::ConfigFileFinder {
    pub fn add_os_folder(&mut self, folder: OSFolder) {
        if let Some(path) = folder.to_path_buf() {
            self.folders.push(path);
        }
    }

    pub fn with_os_folder(mut self, folder: OSFolder) -> Self {
        self.add_os_folder(folder);
        self
    }
}