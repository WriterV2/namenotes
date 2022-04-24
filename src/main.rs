use std::io::{Read, Write};

use clap::Parser;

mod cli;
mod name;

// get path for namenotes location file - path gives by user or home directory
fn get_path(optional_path: Option<std::path::PathBuf>) -> std::path::PathBuf {
    if let Some(o_path) = optional_path {
        o_path
    } else {
        if let Some(home) = dirs::home_dir() {
            home
        } else {
            panic!("Home directory not found");
        }
    }
}

// read the namenotes file (create if it doesn't exist) and parse content
fn read_namenotes(file: std::io::Result<std::fs::File>, path: &std::path::PathBuf) -> String {
    let mut file_content = String::new();
    match file {
        Ok(mut file) => match file.read_to_string(&mut file_content) {
            Ok(_) => file_content,
            Err(readerr) => panic!(
                "Error when trying to read existing namenotes file {:?}",
                readerr
            ),
        },
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                match create_and_open_namenotes_file(path).read_to_string(&mut file_content) {
                    Ok(_) => file_content,
                    Err(create_err) => panic!(
                        "Error when trying to read newly created namesnotes file: {:?}",
                        create_err
                    ),
                }
            }
            other_error => panic!(
                "Error when trying to open existing namesnotes file: {:?}",
                other_error
            ),
        },
    }
}

// create new namenotes file and open in write mode
fn create_and_open_namenotes_file(path: &std::path::PathBuf) -> std::fs::File {
    match std::fs::File::create(path) {
        Ok(_) => match std::fs::File::open(path) {
            Ok(openedfile) => openedfile,
            Err(openerr) => panic!(
                "Error when trying to open newly created namenotes file: {:?}",
                openerr
            ),
        },
        Err(err) => panic!("Error when trying to create namenotes file: {:?}", err),
    }
}

// try to get Names struct from file content - pass potential error forward
fn get_names(content: &String) -> Result<name::Names, std::io::Error> {
    let names = serde_json::from_str(&content)?;
    Ok(names)
}

// add new name to existing names and write to namenotes files
fn write_names_to_file(
    names: Result<name::Names, std::io::Error>,
    new_name: name::Name,
    path: &std::path::PathBuf,
) {
    // get existing names, else create empty names
    let mut file_content = match names {
        Ok(names) => names,
        Err(_) => name::Names(Vec::new()),
    };

    file_content.0.push(new_name);

    let deserialized_names: String = match serde_json::to_string_pretty(&file_content) {
        Ok(s) => s,
        Err(_) => String::new(),
    };

    match std::fs::File::create(&path) {
        Ok(mut newfile) => match newfile.write_all(deserialized_names.as_bytes()) {
            Ok(_) => (),
            Err(err) => panic!(
                "Error when trying to write names to newly created namenotes file: {:?}",
                err
            ),
        },
        Err(err) => panic!(
            "Error when trying to create new namenotes file before writing: {:?}",
            err
        ),
    };
}

fn main() {
    let parsed_args = cli::Cli::parse();

    let mut path: std::path::PathBuf = get_path(parsed_args.path);
    path.push("namenotes.json");
    let f = std::fs::File::open(&path);

    let content = read_namenotes(f, &path);
    let names: Result<name::Names, std::io::Error> = get_names(&content);

    match parsed_args.command {
        // Read Mode
        cli::Commands::Read(x) => {}

        // Write Mode
        cli::Commands::Write(x) => {
            write_names_to_file(names, name::Name::new_from_writeargs(&x), &path);
        }
    };
}
