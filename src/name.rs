use crate::cli;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    pub name: String,
    // meaning of name
    pub meaning: String,
    // language of name
    pub language: String,
    // if name is fictional
    pub fictional: bool,
    // gender: male, female, unisex
    pub gender: cli::Gender,
}

// All names stored
#[derive(Debug, Serialize, Deserialize)]
pub struct Names(pub Vec<Name>);

impl Name {
    // create empty string from None or return value
    pub fn none_to_empty_string(s: &Option<String>) -> String {
        if let Some(slice) = s {
            slice.to_string()
        } else {
            String::new()
        }
    }

    // create Name struct from parsed arguments of write command
    pub fn new_from_writeargs(args: &cli::WriteArgs) -> Name {
        Name {
            name: args.name.to_string(),
            meaning: Self::none_to_empty_string(&args.meaning),
            language: Self::none_to_empty_string(&args.language),
            fictional: args.fictional,
            gender: args.gender,
        }
    }
}
