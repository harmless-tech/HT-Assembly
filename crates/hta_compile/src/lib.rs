mod crafter;

use log::{debug, error, info, trace, warn};
use std::{fs, io::Error, process::exit};

struct CompileData {}

pub fn compile_with_export() {
    //TODO
}

//TODO Returned compiled struct.
pub fn compile(hta_file: &str) {
    let contents: String = intake_file(hta_file);
    let lines: Vec<String> = contents
        .split("\n")
        .map(|s| String::from(s.trim()))
        .collect();
}

fn intake_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(str) => str,
        Err(_) => {
            error!("Failed to import file {}. (Compilation Failed)", path);
            exit(-1);
        }
    }
}
