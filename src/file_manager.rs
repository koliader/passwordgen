use std::fs::{File, OpenOptions};
use std::io::Write;

use crate::argument_parser::Args;
use crate::lib::error::FileManagerError;

pub struct FileManager<'a> {
    passwords: &'a String,
    args: &'a Args,
}

impl FileManager<'_> {
    pub fn new<'a>(passwords: &'a String, args: &'a Args) -> FileManager<'a> {
        FileManager {
            passwords,
            args,
        }
    }

    pub fn save(&self) -> Result<(), FileManagerError> {
        if self.args.save_to_file {
            let mut file = self.create_or_open_file()?;
            if let Err(e) = file.write_all(self.passwords.as_bytes()) {
                return Err(FileManagerError::WriteFileError(e.to_string()));
            }
        }
        Ok(())
    }

    pub fn create_or_open_file(&self) -> Result<File, FileManagerError> {
        match OpenOptions::new().write(true).create(true).truncate(true).open(&self.args.file_name) {
            Ok(v) => Ok(v),
            Err(e) => Err(FileManagerError::OpenFileError(e.to_string())),
        }
    }
}
