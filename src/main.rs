mod logging;
//mod tests; TODO tests.

use log::{debug, error, info, trace, warn};
use std::env;
use std::process::exit;

#[cfg(debug_assertions)]
const DEBUG_BUILD: bool = true;
#[cfg(not(debug_assertions))]
const DEBUG_BUILD: bool = false;

//TODO Allow for optional args. (--binary)
//TODO Built in formatter?
fn main() {
    let mut args = std::env::args();
    let debug = (DEBUG_BUILD && !args.any(|s| s.eq("--release"))) || (!DEBUG_BUILD && args.any(|s| s.eq("--debug")));

    // Logging
    let _logging = logging::setup_log(debug); //TODO Logging returns a result?
    info!("Logging Level Check");
    info!("TRUE");
    warn!("TRUE");
    error!("TRUE");
    debug!("TRUE");
    trace!("TRUE");
    info!("Logging Level Check - Done!");

    let hta_version = match option_env!("CARGO_PKG_VERSION") {
        Some(v) => v,
        None => {
            error!("No version found for hta defaulting to v0.0.1");
            "0.0.1"
        }
    };
    info!("Starting HTA! Version: {}", hta_version);

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
                // Compile and then run a binary.

                // compile_and_run(args[2].as_str());
                warn!("Compile and Run is not implemented yet!");
            }
            else {
                error!("Expected compile, run, or crun!");
            }
        }
        _ => error!("Wrong number of args! Needed 2!"),
    }

    //TODO Remove!
    match hta_compile::compile("assembly-tests/m1/main.ha", "main.ha", debug) {
        Ok(filename) => info!("[SUCCESS] Compiled binary file is at {}.", filename),
        Err(err) => {
            error!("[FAILED] {}", err);
            exit(-1);
        }
    }
    //
}

// fn compile_to_file(hta_file: &str, binary_name: &str) {}

// fn compile_and_run(hta_file: &str) {}

// fn run(binary_file: &str) {}

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
