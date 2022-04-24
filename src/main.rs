use std::io::Read;

use clap::Parser;

mod cli;
mod name;

fn get_home_dir(optional_path: Option<std::path::PathBuf>) -> std::path::PathBuf {
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

fn main() {
    let parsed_args = cli::Cli::parse();

    let mut path: std::path::PathBuf = get_home_dir(parsed_args.path);
    path.push("namenotes.json");
    let f = std::fs::File::open(&path);

    let content = read_namenotes(f, &path);
    let mut names: name::Names = serde_json::from_str(&content).unwrap();

    match parsed_args.command {
        cli::Commands::Read(x) => {}
        cli::Commands::Write(x) => {
            let name1 = name::Name::new_from_writeargs(&x);
            names.0.push(name1);
            println!("{:?}", names);
            println!("\n{}", serde_json::to_string_pretty(&names).unwrap());
        }
    };
}
