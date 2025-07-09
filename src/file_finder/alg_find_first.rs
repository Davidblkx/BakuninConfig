//! Contains the `find_first` algorithm for finding the first file in a directory that matches a given pattern.

use super::{FileFinder, FileFinderError, FindResult};

impl FileFinder {
    /// Finds the first file that matches the specified file name and extensions in the defined folders.
    ///
    /// Folders are searched in the order they were defined.
    ///
    /// Extensions are checked in the order they were defined.
    ///
    /// # Arguments
    /// * `allow_missing` - If true, and no files are found, it will return the first combination.
    pub fn find_first(&self, allow_missing: bool) -> crate::Result<FindResult> {
        self.validate()?;

        let mut first_found: Option<FindResult> = None;

        for folder in &self.folders {
            let base_file = folder.join(self.file_name);
            for ext in &self.extensions {
                let file = base_file.with_extension(ext);
                if file.exists() {
                    let result = FindResult::new(file, ext);
                    return Ok(result);
                } else if allow_missing && first_found.is_none() {
                    // If allow_missing is true, we store the first combination found
                    let result = FindResult::new(file, ext);
                    first_found = Some(result);
                }
            }
        }

        if allow_missing {
            if let Some(result) = first_found {
                return Ok(result);
            }
        }

        Err(FileFinderError::NoFilesFound.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_first_if_allow_missing() {
        let res = FileFinder::new("test_file")
            .with_user_config()
            .with_user_home()
            .with_toml()
            .with_extension("txt")
            .find_first(true)
            .unwrap();

        let expected = super::super::OSDirectory::UserConfig
            .to_path_buf()
            .unwrap()
            .join("test_file.toml");

        assert_eq!(res.path, expected);
    }
}
