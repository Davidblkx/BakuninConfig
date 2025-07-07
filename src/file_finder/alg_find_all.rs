//! Contains the `find_all` algorithm for finding files in a directory that matches a given pattern.

use super::{FileFinder, FileFinderError, FindResult, Result};

impl FileFinder {
    /// Finds the files that matches the specified file name and extensions in the defined folders.
    ///
    /// Folders are searched in the order they were defined.
    ///
    /// Extensions are checked in the order they were defined.
    ///
    /// # Arguments
    /// * `allow_missing` - If true, return all combinations of the file name with the extensions, even if the files do not exist.
    pub fn find_all(&self, allow_missing: bool) -> Result<Vec<FindResult>> {
        self.validate()?;

        let mut files = Vec::new();
        let mut index = 0;

        for folder in &self.folders {
            let base_file = folder.join(self.file_name);
            for ext in &self.extensions {
                let file = base_file.with_extension(ext);
                if allow_missing || file.exists() {
                    files.push(FindResult::new(file, ext).with_index(index));
                    index += 1;
                }
            }
        }

        if files.is_empty() {
            return Err(FileFinderError::NoFilesFound);
        }

        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_all_if_allow_missing() {
        let res = FileFinder::new("test_file")
            .with_user_home()
            .with_user_config()
            .with_toml()
            .find_all(true)
            .unwrap();

        let expected1 = super::super::OSDirectory::UserHome
            .to_path_buf()
            .unwrap()
            .join("test_file.toml");
        let expected2 = super::super::OSDirectory::UserConfig
            .to_path_buf()
            .unwrap()
            .join("test_file.toml");

        assert_eq!(res.len(), 2);
        assert_eq!(res[0].path, expected1);
        assert_eq!(res[1].path, expected2);
    }
}
