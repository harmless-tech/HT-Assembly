mod logging;
//mod tests; TODO tests.

use log::{debug, error, info, trace, warn};
use std::{env, process};

use crate::logging::setup_log;
use std::ops::Add;

//TODO Allow for optional args. (--debug, --binary)
fn main() {
    // Logging
    setup_log();

    info!("Starting HTA!");

    // Args
    let args: Vec<String> = env::args().collect();
    debug!("Entry Args: {}", args.join(", "));
    match args.len() {
        3 => {
            if args[1].eq_ignore_ascii_case("compile") {
                //compile(args[2].as_str());
                //TODO Output compiled 'binary'.
                warn!("Compile is not implemented yet!");
            }
            else if args[1].eq_ignore_ascii_case("run") {
                //TODO Run compile binary.
                //compile(args[2].as_str());
                warn!("Run is not implemented yet!");
            }
            else if args[1].eq_ignore_ascii_case("crun") {
                //TODO Arg checking.
                compile_and_run(args[2].as_str());
                warn!("Compile and Run is not implemented yet!");
            }
            else {
                error!("Expected compile, run, or crun!");
            }
        },
        _ => error!("Wrong number of args! Needed 2!")
    }
}

fn compile_to_file(hta_file: &str, binary_name: &str) {}

fn compile_and_run(hta_file: &str) {}

fn run(binary_file: &str) {}

/*fn compile(location: &str) {
    let file: String = file_io::import_file(location);
    if file.eq("") {
        process::exit(1); //CODE 1: File to compile was empty or not found.
    }

    let data: HTADatabase = hta_compile::compile(file.as_str());

    //TODO Remove!!
    debug!("Database print out:\n{:#?}", data);
}

fn run() {}*/
