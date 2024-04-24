use std::time::Duration;

use crate::argument_parser::Args;

pub struct Printer<'a> {
    args: &'a Args,
    passwords: &'a String,
}

impl Printer<'_> {
    pub fn new<'a>(passwords: &'a String, args: &'a Args) -> Printer<'a> {
        Printer {
            passwords,
            args,
        }
    }
    pub fn print_passwords(&self) {
        if !self.args.save_to_file {
            println!("{}", self.passwords)
        }
    }

    pub fn print_time(&self, duration: Duration) {
        println!("Time elapsed in generating passwords is: {:?}", duration);
    }
}
