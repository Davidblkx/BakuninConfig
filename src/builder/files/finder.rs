use std::path::PathBuf;

use super::file::ValueFileHandler;

/// Helper to locate configuration files from a list of folders and supported extensions
pub struct ConfigFileFinder {
    pub file_name: String,
    pub folders: Vec<PathBuf>,
    pub extensions: Vec<String>,
    add_supported_extensions: bool,
}

impl ConfigFileFinder {
    /// Creates a new ConfigFileFinder
    /// 
    /// The file_name is the name of the file to find.
    /// The folders is a list of folders where to search for the file.
    /// The extensions is a list of supported extensions.
    pub fn new(file_name: String, folders: Vec<PathBuf>, extensions: Vec<String>) -> Self {
        Self {
            file_name,
            folders,
            extensions,
            add_supported_extensions: false,
        }
    }

    pub fn for_file(file_name: String) -> Self {
        Self {
            file_name,
            folders: Vec::new(),
            extensions: Vec::new(),
            add_supported_extensions: false,
        }
    }

    pub fn with_folder(mut self, folder: PathBuf) -> Self {
        self.folders.push(folder);
        self
    }

    pub fn with_folders(mut self, folders: Vec<PathBuf>) -> Self {
        self.folders.extend(folders);
        self
    }

    pub fn with_extension(mut self, extension: String) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn with_extensions(mut self, extensions: Vec<String>) -> Self {
        self.extensions.extend(extensions);
        self
    }

    /// Add extensions supported by loaded features
    pub fn add_supported_extensions(&mut self) {
        if self.add_supported_extensions {
            return;
        }

        let mut _extensions = Vec::new();

        #[cfg(feature = "toml")]
        _extensions.extend(super::handler_toml::TomlConfigHandler::extensions());

        #[cfg(feature = "json")]
        _extensions.extend(super::handler_json::JsonConfigHandler::extensions());

        #[cfg(feature = "yaml")]
        _extensions.extend(super::handler_yaml::YamlConfigHandler::extensions());

        #[cfg(feature = "json5")]
        _extensions.extend(super::handler_json5::Json5ConfigHandler::extensions());

        if _extensions.is_empty() {
            log::warn!("No supported extensions found. Please enable one of the following features: toml, json, yaml, json5");
        }

        self.extensions.extend(_extensions);
        self.add_supported_extensions = true;
    }

    /// Add extensions supported by loaded features
    pub fn with_supported_extensions(mut self) -> Self {
        self.add_supported_extensions();
        self
    }

    /// Returns the path of the first file found
    /// 
    /// The file is searched in the folders and with the extensions defined in the ConfigFileFinder.
    /// If no file is found, None is returned.
    pub fn find(&self) -> Option<PathBuf> {
        for folder in &self.folders {
            for extension in &self.extensions {
                let mut path = folder.clone();
                path.push(format!("{}.{}", self.file_name, extension));
                let exists = match path.try_exists() {
                    Ok(exists) => exists,
                    Err(_) => continue,
                };
                if exists {
                    return Some(path);
                }
            }
        }
        None
    }

    /// Returns the path of all files found
    /// 
    /// The files are searched in the folders and with the extensions defined in the ConfigFileFinder.
    /// If no file is found, an empty vector is returned.
    pub fn find_all(&self) -> Vec<PathBuf> {
        let mut files = Vec::new();
        for folder in &self.folders {
            for extension in &self.extensions {
                let mut path = folder.clone();
                path.push(format!("{}.{}", self.file_name, extension));
                let exists = match path.try_exists() {
                    Ok(exists) => exists,
                    Err(_) => continue,
                };
                if exists {
                    files.push(path);
                }
            }
        }
        files
    }

    /// Returns the path of the first file found or the path of the first file with the first extension
    /// 
    /// ex: if the file_name is "config" and the extensions are "toml" and "yaml", 
    /// the method will return the first file found with the extensions "config.toml" or "config.yaml".
    /// If no file is found, config.toml is returned.
    pub fn find_or_first(&self) -> Option<PathBuf> {
        if let Some(path) = self.find() {
            Some(path)
        } else {
            match self.folders.first() {
                Some(folder) => {
                    match self.extensions.first() {
                        Some(extension) => {
                            let mut path = folder.clone();
                            path.push(format!("{}.{}", self.file_name, extension));
                            Some(path)
                        },
                        None => None,
                    }
                },
                None => None,
            }
        }
    }
}