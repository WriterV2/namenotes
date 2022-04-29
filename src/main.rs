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

// open exisitng namenotes file in read mode
fn open_existing_namenotes(path: &std::path::PathBuf) -> std::fs::File {
    let old_file = match std::fs::File::open(&path) {
        Ok(file) => file,
        Err(openerr) => match openerr.kind() {
            std::io::ErrorKind::NotFound => create_new_namenotes_to_read(&path),
            other_err => panic!(
                "Error when trying to read exisiting namenotes file {:?}",
                other_err
            ),
        },
    };
    old_file
}

// create new namenotes file with empty names list and open it in read mode
fn create_new_namenotes_to_read(path: &std::path::PathBuf) -> std::fs::File {
    let new_file = create_new_namenotes(&path);
    let empty_names = name::Names(Vec::new());

    empty_names.write_to_json(new_file);

    match std::fs::File::open(&path) {
        Ok(new_file_to_read) => new_file_to_read,
        Err(read_err) => panic!(
            "Error when trying to read newly created namenotes file with empty names list: {:?}",
            read_err
        ),
    }
}

// create new namenotes file in write mode
fn create_new_namenotes(path: &std::path::PathBuf) -> std::fs::File {
    match std::fs::File::create(&path) {
        Ok(file) => file,
        Err(create_err) => panic!(
            "Error when trying to create new namenotes file {:?}",
            create_err
        ),
    }
}

fn main() {
    // parsed CLI arguments: WriteArgs for write command, ReadArgs for read command
    let parsed_args = cli::Cli::parse();

    // path specified by user with path argument or home directory
    let mut path: std::path::PathBuf = get_path(parsed_args.path);
    path.push("namenotes.json");

    match parsed_args.command {
        // Read Mode
        cli::Commands::Read(x) => {
            let all_names = name::Names::new_from_json_file(open_existing_namenotes(&path));
            // Example for returning a list of names filtered by arguments of read command
            println!("{}", all_names.filtered_list(x));
        }

        // Write Mode
        cli::Commands::Write(x) => {
            // Example for creating Names struct from arguments
            let new_names = name::Names::new_from_json(name::Names::args_to_json(&x));
            // Example for creating Names struct from existing file
            let mut all_names = name::Names::new_from_json_file(open_existing_namenotes(&path));
            // Example for appending new names to old names
            all_names = all_names.append_new_names(new_names);
            // Example for writing all names to file
            all_names.write_to_json(create_new_namenotes(&path));
        }
    };
}
