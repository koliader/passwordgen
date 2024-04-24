use clap::Parser;

use crate::lib::error::ArgumentParserError;

#[derive(Parser, Debug)]
#[command(version = "1.0.0", about, long_about = None)]
pub struct Args {
    #[arg(short, long, required = true)]
    pub key: Vec<String>,

    #[arg(short, long)]
    pub save_to_file: bool,

    #[arg(short = 'n', long, default_value = "passwords.txt")]
    pub file_name: String,
}

pub struct ArgParser {
    pub args: Args,
}

impl ArgParser {
    pub fn new() -> Result<ArgParser, ArgumentParserError> {
        let args = Self::parse()?;
        Ok(ArgParser {
            args
        })
    }
    fn parse() -> Result<Args, ArgumentParserError> {
        Args::try_parse().map_err(|e| {
            ArgumentParserError::ParsingError(e.to_string())
        })
    }
}