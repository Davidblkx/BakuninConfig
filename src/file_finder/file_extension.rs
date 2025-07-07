/// Represents the file extension used in configuration files.
/// It must NOT include the leading dot (e.g., "json", "toml").
#[derive(Debug, Clone)]
pub enum FileExtension {
    Custom(&'static str),
    #[cfg(feature = "toml")]
    Toml,
    #[cfg(feature = "json")]
    Json,
}

impl From<&'static str> for FileExtension {
    fn from(ext: &'static str) -> Self {
        match ext {
            #[cfg(feature = "toml")]
            ".toml" => FileExtension::Toml,
            #[cfg(feature = "json")]
            ".json" => FileExtension::Json,
            _ => FileExtension::Custom(ext),
        }
    }
}

impl Into<&'static str> for FileExtension {
    fn into(self) -> &'static str {
        self.as_str()
    }
}

impl FileExtension {
    pub fn as_str(&self) -> &'static str {
        match self {
            FileExtension::Custom(ext) => ext,
            #[cfg(feature = "toml")]
            FileExtension::Toml => "toml",
            #[cfg(feature = "json")]
            FileExtension::Json => "json",
        }
    }

    pub fn match_path<P: AsRef<std::path::Path>>(&self, path: P) -> bool {
        let ext = path.as_ref().extension().and_then(|s| s.to_str());
        match self {
            #[cfg(feature = "toml")]
            FileExtension::Toml => ext == Some("toml"),
            #[cfg(feature = "json")]
            FileExtension::Json => ext == Some("json"),
            _ => false,
        }
    }
}

impl super::FileFinder {
    #[cfg(feature = "toml")]
    /// Search for TOML files.
    pub fn with_toml(self) -> Self {
        self.with_extension(FileExtension::Toml.as_str())
    }

    #[cfg(feature = "json")]
    /// Search for JSON files.
    pub fn with_json(self) -> Self {
        self.with_extension(FileExtension::Json.as_str())
    }
    
    /// Add all built-in supported extensions.
    pub fn with_supported_extensions(self) -> Self {
        let mut finder = self;
        #[cfg(feature = "toml")]
        {
            finder = finder.with_toml();
        }
        #[cfg(feature = "json")]
        {
            finder = finder.with_json();
        }
        finder
    }
}
