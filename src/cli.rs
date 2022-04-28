use clap::{ArgEnum, Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

// Top-level CLI arguments
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    // Subcommands: "write", "read"
    #[clap(subcommand)]
    pub command: Commands,

    // argument with default value: "-p", "--path"
    #[clap(
        short,
        long,
        help = "Optional path to directory containing namenotes.json - default: home directory"
    )]
    pub path: Option<std::path::PathBuf>,
}

// Arguments for "write" subcommand
#[derive(Debug, Args, Serialize, Deserialize)]
pub struct WriteArgs {
    // required argument: "-n", "--name"
    #[clap(short, long, help = "Name to note down")]
    pub name: String,

    // optional argument: "-l", "--language"
    #[clap(short, long, help = "Language of name")]
    #[serde(default)]
    pub language: Option<String>,

    // optional argument: "-m", "--meaning"
    #[clap(short, long, help = "Meaning of name")]
    #[serde(default)]
    pub meaning: Option<String>,

    // optional argument: "-g", "--gender"
    #[clap(
        short,
        long,
        arg_enum,
        help = "Gender of name",
        default_value = "unisex"
    )]
    pub gender: Gender,

    // boolean argument: "-f", "--fictional"
    #[clap(short, long, help = "If name is fictional")]
    pub fictional: bool,
}

// display name - only show attributes, if available
impl std::fmt::Display for WriteArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("'{}'\n", self.name);
        let language = if let Some(l) = &self.language {
            format!("Language: {}\n", l.to_string())
        } else {
            String::new()
        };
        let meaning = if let Some(m) = &self.meaning {
            format!("Meaning: {}\n", m.to_string())
        } else {
            String::new()
        };
        let gender = format!(
            "Gender: {}\n",
            match self.gender {
                Gender::Male => "Masculine".to_string(),
                Gender::Female => "Feminine".to_string(),
                _ => "Unisex".to_string(),
            }
        );

        let fictional = match &self.fictional {
            true => String::from("(fictional)\n"),
            false => String::new(),
        };
        write!(f, "{}{}{}{}{}", name, language, meaning, gender, fictional)
    }
}

// Arguments for "read" subcommand
#[derive(Debug, Args, Serialize, Deserialize)]
pub struct ReadArgs {
    // optional argument: "-n", "--name"
    #[clap(short, long, help = "Filter by name")]
    #[serde(default)]
    pub name: Option<String>,

    // optional argument: "-l", "--language"
    #[clap(short, long, help = "Filter by language of name")]
    #[serde(default)]
    pub language: Option<String>,

    // optional argument: "-m", "--meaning"
    #[clap(
        short,
        long,
        help = "Filter by sequence of characters in meaning of name"
    )]
    #[serde(default)]
    pub meaning: Option<String>,

    // optional argument: "-g", "--gender"
    #[clap(short, long, arg_enum, help = "Filter by gender")]
    #[serde(default)]
    pub gender: Option<Gender>,

    // optional argument: "--length"
    #[clap(long, help = "Filter by length of name")]
    #[serde(skip)]
    pub length: Option<u8>,

    // optional argument: "-c", "--contains"
    #[clap(
        short,
        long,
        help = "Filter by sequence of characters name has to contain"
    )]
    #[serde(skip)]
    pub contains: Option<String>,

    // optional argument: "--contains-letter"
    #[clap(long, help = "Filter by character name has to contain")]
    #[serde(skip)]
    pub contains_letter: Option<char>,

    // boolean argument: "-f", "--fictional"
    #[clap(short, long, help = "Filter by whether name is fictional")]
    pub fictional: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Write(WriteArgs),
    Read(ReadArgs),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Unisex,
}
