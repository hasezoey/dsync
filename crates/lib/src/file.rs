use std::path::{Path, PathBuf};

use crate::{
    error::{Error, IOErrorToError, Result},
    FILE_SIGNATURE,
};

#[derive(Debug)]
pub struct MarkedFile {
    /// File contents to read / to write
    file_contents: String,
    /// Path of the resulting file
    pub path: PathBuf,
    modified: bool,
}

impl MarkedFile {
    /// Tries to open the file at `path` and read contents into `file_contents`
    pub fn new(path: PathBuf) -> Result<MarkedFile> {
        Ok(MarkedFile {
            path: path.clone(),
            file_contents: if !path.exists() {
                "".to_owned()
            } else {
                std::fs::read_to_string(&path).attach_path_err(&path)?
            },
            modified: false,
        })
    }

    pub fn get_file_contents(&self) -> &str {
        &self.file_contents
    }

    pub fn is_modified(&self) -> bool {
        self.modified
    }

    pub fn is_empty(&self) -> bool {
        self.file_contents.is_empty() || self.file_contents.trim() == FILE_SIGNATURE
    }

    pub fn has_use_stmt(&self, use_name: &str) -> bool {
        self.file_contents.contains(&format!("pub use {use_name};"))
    }

    pub fn has_mod_stmt(&self, mod_name: &str) -> bool {
        self.file_contents.contains(&format!("pub mod {mod_name};"))
    }

    /// Change `file_contents` to be `new_content`, sets `modified`
    pub fn change_file_contents(&mut self, new_content: String) {
        if !self.modified && self.file_contents != new_content {
            self.modified = true;
        }
        self.file_contents = new_content;
    }

    /// Change `file_contents` to be `new_content` always, does not set `modified`
    pub fn change_file_contents_no_modify(&mut self, new_content: String) {
        self.file_contents = new_content;
    }

    pub fn add_use_stmt(&mut self, use_name: &str) {
        self.file_contents = self.file_contents.trim().to_string();
        if !self.file_contents.is_empty() {
            self.file_contents.push('\n');
        }
        self.file_contents
            .push_str(&format!("pub use {use_name};\n"));
        self.modified = true;
    }

    pub fn add_mod_stmt(&mut self, mod_name: &str) {
        self.file_contents = self.file_contents.trim().to_string();
        if !self.file_contents.is_empty() {
            self.file_contents.push('\n');
        }
        self.file_contents
            .push_str(&format!("pub mod {mod_name};\n"));
        self.modified = true;
    }

    pub fn remove_use_stmt(&mut self, mod_name: &str) {
        let content_to_remove = &format!("pub use {mod_name};");
        if self.file_contents.contains(content_to_remove) {
            self.file_contents = self
                .file_contents
                .replace(content_to_remove, "")
                .trim()
                .to_string();
            self.modified = true;
        }
    }

    pub fn remove_mod_stmt(&mut self, mod_name: &str) {
        let content_to_remove = &format!("pub mod {mod_name};");
        if self.file_contents.contains(content_to_remove) {
            self.file_contents = self
                .file_contents
                .replace(content_to_remove, "")
                .trim()
                .to_string();
            self.modified = true;
        }
    }

    pub fn ensure_use_stmt(&mut self, use_name: &str) {
        if !self.has_use_stmt(use_name) {
            self.add_use_stmt(use_name)
        }
    }

    pub fn ensure_mod_stmt(&mut self, mod_name: &str) {
        if !self.has_mod_stmt(mod_name) {
            self.add_mod_stmt(mod_name)
        }
    }

    pub fn has_file_signature(&self) -> bool {
        // the reason we consider filelength=0 as having a file signature is because
        // the whole purpose of file signatures is to prevent writing to files which aren't generated
        // and if a file's length is 0, then it's safe to write to this file!
        // :)
        self.file_contents.is_empty()
            || self
                .file_contents
                .starts_with(crate::parser::FILE_SIGNATURE)
    }

    /// Ensure that the file, if it already exists, has the dsync file signature
    /// to prevent accidental overwriting of non-dsync files
    pub fn ensure_file_signature(&self) -> Result<()> {
        if !self.has_file_signature() {
            return Err(Error::no_file_signature(format!("Expected file '{path:#?}' to have file signature ('{sig}') -- you might be accidentally overwriting files that weren't generated!", path=self.path, sig=crate::parser::FILE_SIGNATURE)));
        }

        Ok(())
    }

    pub fn write(&self) -> Result<()> {
        std::fs::write(&self.path, &self.file_contents).attach_path_err(&self.path)
    }

    pub fn delete(self) -> Result<PathBuf> {
        std::fs::remove_file(&self.path).attach_path_err(&self.path)?;

        Ok(self.path)
    }
}

impl AsRef<Path> for MarkedFile {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}
