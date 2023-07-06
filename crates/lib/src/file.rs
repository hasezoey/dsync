use std::path::PathBuf;

use crate::error::{IOErrorToError, Result};

pub struct MarkedFile {
    pub file_contents: String,
    pub path: PathBuf,
}

impl MarkedFile {
    pub fn new(path: PathBuf) -> Result<MarkedFile> {
        Ok(MarkedFile {
            path: path.clone(),
            file_contents: if !path.exists() {
                std::fs::write(&path, "").attach_path_err(&path)?;
                "".to_string()
            } else {
                std::fs::read_to_string(&path).attach_path_err(&path)?
            },
        })
    }

    pub fn has_use_stmt(&self, use_name: &str) -> bool {
        self.file_contents.contains(&format!("pub use {use_name};"))
    }

    pub fn has_mod_stmt(&self, mod_name: &str) -> bool {
        self.file_contents.contains(&format!("pub mod {mod_name};"))
    }

    pub fn add_use_stmt(&mut self, use_name: &str) {
        self.file_contents = self.file_contents.trim().to_string();
        if !self.file_contents.is_empty() {
            self.file_contents.push('\n');
        }
        self.file_contents
            .push_str(&format!("pub use {use_name};\n"))
    }

    pub fn add_mod_stmt(&mut self, mod_name: &str) {
        self.file_contents = self.file_contents.trim().to_string();
        if !self.file_contents.is_empty() {
            self.file_contents.push('\n');
        }
        self.file_contents
            .push_str(&format!("pub mod {mod_name};\n"))
    }

    pub fn remove_use_stmt(&mut self, mod_name: &str) {
        let content_to_remove = &format!("pub use {mod_name};");
        if self.file_contents.contains(content_to_remove) {
            self.file_contents = self
                .file_contents
                .replace(content_to_remove, "")
                .trim()
                .to_string();
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

    pub fn ensure_file_signature(&self) {
        if !self.has_file_signature() {
            panic!("Expected file '{path:#?}' to have file signature ('{sig}') -- you might be accidentally overwriting files that weren't generated!", path=self.path, sig=crate::parser::FILE_SIGNATURE)
        }
    }

    pub fn write(&self) -> Result<()> {
        std::fs::write(&self.path, &self.file_contents).attach_path_err(&self.path)
    }

    pub fn delete(self) -> Result<()> {
        std::fs::remove_file(&self.path).attach_path_err(&self.path)
    }
}
