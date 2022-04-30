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

impl WriteArgs {
    // whether this name and its attributes matches with the filter arguments
    pub fn matches_readargs(&self, args: &ReadArgs) -> bool {
        // if name was specified and if it matches with this name
        let name_matches = if let Some(args_name) = args.name.as_ref() {
            Some(&self.name == args_name)
        } else {
            None
        };

        // if a letter that name must contain was specified and if this name contains it
        let name_contains_char = if let Some(args_char) = args.contains_letter {
            Some(self.name.contains(args_char))
        } else {
            None
        };

        // if a sequence of letters that name must contain was specified and if this name contains it
        let name_contains_string = if let Some(args_string) = args.contains.as_ref() {
            Some(self.name.contains(args_string))
        } else {
            None
        };

        // if the language of the name was specified, if it was specified in this name and if they both match
        let language_matches = if let Some(args_lang) = args.language.as_ref() {
            if let Some(lang) = &self.language {
                Some(lang == args_lang)
            } else {
                Some(false)
            }
        } else {
            None
        };

        // if the meaning of the name was specified, if it was specified in this name and if they both match
        let meaning_matches = if let Some(args_meaning) = args.meaning.as_ref() {
            if let Some(meaning) = &self.meaning {
                Some(meaning.contains(&*args_meaning))
            } else {
                Some(false)
            }
        } else {
            None
        };

        // if the gender of the name was specified and if the specified gender accepts this name's gender
        // Unisex <- Unisex | Male <- Unisex, Male | Female <- Unisex, Female
        let gender_matches = if let Some(args_gender) = args.gender {
            match (self.gender, args_gender) {
                (Gender::Male, Gender::Female) => Some(false),
                (Gender::Female, Gender::Male) => Some(false),
                (Gender::Male, Gender::Unisex) => Some(false),
                (Gender::Female, Gender::Unisex) => Some(false),
                _ => Some(true),
            }
        } else {
            None
        };

        // if name was specified as fictional and if this name is fictional
        let fictional_matches = if args.fictional {
            Some(self.fictional == args.fictional)
        } else {
            None
        };

        // the values of all specified attributes
        let mut specified_args = Vec::new();

        for arg in [
            name_matches,
            name_contains_char,
            name_contains_string,
            language_matches,
            meaning_matches,
            gender_matches,
            fictional_matches,
        ]
        .iter()
        {
            if let Some(a) = arg {
                specified_args.push(*a);
            }
        }

        // if no attribute was specified, return all names
        if specified_args.is_empty() {
            true
        } else {
            // name is a match, if all specified attributes match
            !specified_args.contains(&false)
        }
    }
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
