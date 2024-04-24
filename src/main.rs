use std::time::Instant;

use crate::file_manager::FileManager;
use crate::lib::converter::Converter;
use crate::lib::error::print_error;
use crate::lib::printer::Printer;
use crate::password_generator::PasswordGenerator;

mod argument_parser;
mod password_generator;

mod lib {
    pub mod error;
    pub mod printer;
    pub mod converter;
}

mod file_manager;


fn main() {
    let start = Instant::now();
    let argument_parser = argument_parser::ArgParser::new().unwrap_or_else(|e| {
        print_error(e)
    });

    let mut password_generator = PasswordGenerator::new(&argument_parser.args);
    let passwords = password_generator.generate();

    let converter = Converter {};
    let string_passwords = converter.convert_arr_to_string(&passwords);

    let file_manager = FileManager::new(&string_passwords, &argument_parser.args);
    file_manager.save().unwrap_or_else(|e| {
        print_error(e)
    });

    let printer = Printer::new(&string_passwords, &argument_parser.args);
    printer.print_passwords();
    let duration = start.elapsed();
    printer.print_time(duration);
}