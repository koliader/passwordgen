use std::process::exit;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArgumentParserError {
    #[error("error to parse arguments: {0}")]
    ParsingError(String)
}

#[derive(Error, Debug)]
pub enum FileManagerError {
    #[error("open file error: {0}")]
    OpenFileError(String),

    #[error("error to write file: {0}")]
    WriteFileError(String),
}

pub fn print_error<T: std::fmt::Display>(e: T) -> ! {
    eprintln!("Error: {}", e);
    exit(0)
}