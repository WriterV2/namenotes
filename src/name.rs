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
