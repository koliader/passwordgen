use std::collections::HashSet;

use crate::argument_parser::Args;

pub struct PasswordGenerator<'a> {
    args: &'a Args,
}

impl PasswordGenerator<'_> {
    pub fn new(args: &Args) -> PasswordGenerator {
        PasswordGenerator {
            args,
        }
    }
    pub fn generate(&mut self) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        // single keyword
        for key in self.args.key.iter() {
            let mut variants = self.convert_to_variants(String::from(key));
            res.append(&mut variants)
        }
        // with two keywords
        for key in self.args.key.iter() {
            let mut keys = self.generate_with_second_key(key);
            res.append(&mut keys);
        }
        let mut unique_res = HashSet::new();
        unique_res.extend(res.into_iter());
        let res: Vec<String> = unique_res.into_iter().collect();
        res
    }
    fn generate_with_second_key(&self, key: &String) -> Vec<String> {
        let mut res = Vec::new();
        let variants = self.convert_to_variants(String::from(key));
        for second_key in self.args.key.iter() {
            let second_variants = self.convert_to_variants(String::from(second_key));
            // iter each first key variant
            for variant in variants.iter() {
                // iter each second key variant
                for second_variant in second_variants.iter() {
                    // join two keys
                    res.push(format!("{}{}", variant, second_variant));
                }
            }
        }
        res
    }
    fn convert_to_variants(&self, key: String) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        res.push(key.to_uppercase());
        res.push(key.to_lowercase());
        res.push(format!("{}{}", &key[0..1].to_uppercase(), &key[1..]));
        res.push(key.clone());
        res
    }
}
