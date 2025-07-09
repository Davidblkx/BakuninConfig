//! Contains the `find_all_or_first` algorithm for finding files in a directory that matches a given pattern.

use super::{FileFinder, FileFinderError, FindResult};

impl FileFinder {
    /// Finds the files that matches the specified file name and extensions in the defined folders.
    /// If no files are found, it returns the first file found with the first extension.
    ///
    /// Folders are searched in the order they were defined.
    ///
    /// Extensions are checked in the order they were defined.
    pub fn find_all_or_first(&self) -> crate::Result<Vec<FindResult>> {
        self.validate()?;

        let mut files = Vec::new();
        let mut first_file: Option<FindResult> = None;
        let mut index = 0;

        for folder in &self.folders {
            let base_file = folder.join(self.file_name);
            for ext in &self.extensions {
                let file = base_file.with_extension(ext);
                if file.exists() {
                    files.push(FindResult::new(file, ext).with_index(index));
                    index += 1;
                } else if first_file.is_none() {
                    // Store the first file found with the first extension
                    first_file = Some(FindResult::new(file, ext));
                }
            }
        }

        if files.is_empty() {
            if let Some(first) = first_file {
                // If no files were found, return the first file found with the first extension
                files.push(first.with_index(0));
            } else {
                return Err(FileFinderError::NoFilesFound.into());
            }
        }

        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_first_if_no_file_found() {
        let res = FileFinder::new("test_file")
            .with_user_home()
            .with_user_config()
            .with_toml()
            .find_all_or_first()
            .unwrap();

        let expected1 = super::super::OSDirectory::UserHome
            .to_path_buf()
            .unwrap()
            .join("test_file.toml");

        assert_eq!(res.len(), 1);
        assert_eq!(res[0].path, expected1);
    }
}
