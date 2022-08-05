use crate::cli;
use serde::{Deserialize, Serialize};
use std::fmt::Write;

// All names stored
#[derive(Debug, Serialize, Deserialize)]
pub struct Names(pub Vec<cli::WriteArgs>);

impl Names {
    // name in cli arguments to json
    pub fn args_to_json<T: Serialize + for<'a> Deserialize<'a> + clap::Args>(args: &T) -> String {
        match serde_json::to_string_pretty(&args) {
            Ok(s) => s,
            Err(err) => panic!("Args to Json {:?}", err),
        }
    }

    // get names from json file
    pub fn new_from_json_file(file: std::fs::File) -> Self {
        match serde_json::from_reader(std::io::BufReader::new(file)) {
            Ok(names) => names,
            Err(err) => panic!("New From Json File {:?}", err),
        }
    }

    // get names from json
    pub fn new_from_json(json: String) -> Self {
        match serde_json::from_str(&json) {
            Ok(names) => Names(vec![names]),
            Err(err) => {
                if err.is_eof() {
                    match serde_json::from_str(&json) {
                        Ok(name) => Names(vec![name]),
                        Err(eof_err) => panic!("New From Json - Single Name {:?}", eof_err),
                    }
                } else {
                    panic!("New From Json - Multiple Names {:?}", err)
                }
            }
        }
    }

    // append other names to this
    pub fn append_new_names(mut self, mut new_names: Names) -> Self {
        self.0.append(&mut new_names.0);
        self
    }

    // write these names to file
    pub fn write_to_json<W: std::io::Write>(&self, writer: W) {
        match serde_json::to_writer_pretty(writer, self) {
            Ok(_) => (),
            Err(err) => panic!("Write to Json {:?}", err),
        }
    }

    // filter names by arguments of read command
    pub fn filtered_list(self, args: cli::ReadArgs) -> Names {
        let mut filtered_list = Names(Vec::new());
        for name in self.0.into_iter() {
            if name.matches_readargs(&args) {
                filtered_list.0.push(name);
            }
        }
        filtered_list
    }
}

// display names and show how many names found
impl std::fmt::Display for Names {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        let mut count = 0;
        for name in self.0.iter() {
            //string.push_str(&format!("{}\n", name));
            writeln!(string, "{}", name).unwrap();
            count += 1;
        }
        write!(f, "\n{} Names found: \n\n{}", count, string)
    }
}
