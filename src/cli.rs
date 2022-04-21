use clap::{ArgEnum, Args, Parser, Subcommand};

// Top-level CLI arguments
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    // Subcommands: "write", "read"
    #[clap(subcommand)]
    pub command: Commands,
}

// Arguments for "write" subcommand
#[derive(Debug, Args)]
pub struct WriteArgs {
    // required argument: "-n", "--name"
    #[clap(short, long)]
    pub name: String,

    // optional argument: "-l", "--language"
    #[clap(short, long)]
    pub language: Option<String>,

    // optional argument: "-m", "--meaning"
    #[clap(short, long)]
    pub meaning: Option<String>,

    // optional argument: "-g", "--gender"
    #[clap(short, long, arg_enum)]
    pub gender: Option<Gender>,
}

#[derive(Debug, Args)]
pub struct ReadArgs {
    // optional argument: "-n", "--name"
    #[clap(short, long)]
    pub name: Option<String>,

    // optional argument: "-l", "--language"
    #[clap(short, long)]
    pub language: Option<String>,

    // optional argument: "-m", "--meaning"
    #[clap(short, long)]
    pub meaning: Option<String>,

    // optional argument: "-g", "--gender"
    #[clap(short, long, arg_enum)]
    pub gender: Option<Gender>,
}

#[derive(Subcommand)]
pub enum Commands {
    Write(WriteArgs),
    Read(ReadArgs),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Gender {
    Male,
    Female,
    Unisex,
}
