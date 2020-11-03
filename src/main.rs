mod logging;
mod tests;

use log::{debug, error, info, trace, warn};
use std::{env, process};

use crate::logging::setup_log;
use hta_shared::{file_io, hta_database::HTADatabase};
use std::ops::Add;

/*
 * Commands:
 * - compile [FILE_NAME] //TODO Allow for debug builds.
 * - run [FILE_NAME]
 * //TODO Allow for build files and multiple files to be compiled together.
 * //TODO Allow for outputted compiled file.
 */

fn main() {
    // Logging
    setup_log();

    info!("Starting HTA!");

    // Args
    let args: Vec<String> = env::args().collect();
    debug!("Entry Args: {}", args.join(", "));
    match args.len() {
        //TODO TEST Might need to switch back to 2.
        3 => {
            if args[1].eq("compile") {
                compile(args[2].as_str());
            //TODO Output compiled 'binary'.
            }
            else if args[1].eq("run") {
                //TODO Get compile result and run it.
                //compile(args[2].as_str());
                warn!("Run is not implemented yet!");
            }
            else {
                error!("Expected compile or run!");
            }
        },
        _ => error!("Wrong number of args! Needed 2!")
    }
}

//TODO Return a compiled file!
fn compile(location: &str) {
    let file: String = file_io::import_file(location);
    if file.eq("") {
        process::exit(1); //CODE 1: File to compile was empty or not found.
    }

    let data: HTADatabase = hta_compile::compile(file.as_str());

    //TODO Remove!!
    debug!("Database print out:\n{:#?}", data);

    *data.frames.get("main").unwrap().instructions.get(3).unwrap()
    //
}

fn run() {}
