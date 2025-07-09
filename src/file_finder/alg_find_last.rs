//! Contains the `find_last` algorithm for finding the last file in a directory that matches a given pattern.

use super::{FileFinder, FileFinderError, FindResult};

impl FileFinder {
    /// Finds the last file that matches the specified file name and extensions in the defined folders.
    ///
    /// Folders are searched in the order they were defined.
    ///
    /// Extensions are checked in the order they were defined.
    ///
    /// # Arguments
    /// * `allow_missing` - If true, and no files are found, it will return the last combination.
    pub fn find_last(&self, allow_missing: bool) -> crate::Result<FindResult> {
        self.validate()?;

        // Last existing file found
        let mut last_found: Option<FindResult> = None;
        // Last file that would be returned if allow_missing is true
        let mut last_result: Option<FindResult> = None;

        for folder in &self.folders {
            let base_file = folder.join(self.file_name);
            for ext in &self.extensions {
                let file = base_file.with_extension(ext);
                if file.exists() {
                    last_found = Some(FindResult::new(file, ext));
                } else if allow_missing && last_found.is_none() {
                    last_result = Some(FindResult::new(file, ext));
                }
            }
        }

        if let Some(result) = last_found {
            return Ok(result);
        } else if allow_missing {
            if let Some(result) = last_result {
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
    fn returns_last_if_allow_missing() {
        let res = FileFinder::new("test_file")
            .with_user_config()
            .with_user_home()
            .with_toml()
            .with_extension("txt")
            .find_last(true)
            .unwrap();

        let expected = super::super::OSDirectory::UserHome
            .to_path_buf()
            .unwrap()
            .join("test_file.txt");

        assert_eq!(res.path, expected);
    }
}
