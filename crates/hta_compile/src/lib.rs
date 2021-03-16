mod compiler;
mod crafter;
mod writer;

use hta_shared::{hfs, version::parse_version_str, DebugData, MetaData, Program};
use log::{debug, error, info, trace, warn};
use std::{
    fs,
    fs::File,
    io::{Seek, SeekFrom},
    path::PathBuf,
};

/* Steps:
 * Take in main file and process info.
 * Take in other files and process info.
 * Make sure files do not import each other.
 * Process files.
 * Do link checks.
 * Create binary.
 */

static BINARY_PATH: &str = "build/bin/";
static DEFAULT_FILE_NAME: &str = "main";

struct WriteData {
    build_data: (String, String), // Right now this is just a file name. (File Name, EMPTY)
    compiler_version: (u64, u64, u64),
    debug_data: Option<DebugData>,
    metadata: MetaData,
    program: Program,
}

// Returns the binary file name on success.
//TODO Allow for multiple errors to be returned.
pub fn compile(hta_file: &str, dbg: bool) -> Result<String, String> {
    let compiler_version = match parse_version_str(option_env!("CARGO_PKG_VERSION").unwrap()) {
        Some(v) => {
            info!("HTA Compiler Version: {}.{}.{}", v.0, v.1, v.2);
            v
        }
        None => {
            error!("No compiler version found defaulting to v0.0.1");
            (0, 0, 1)
        }
    };

    let contents = file::intake(hta_file)?;

    debug!("Size: {}", contents.clone().len());
    debug!("\n{}", contents.clone());

    let stripped_contents = compiler::remove_comments(contents)?;

    debug!("Size: {}", stripped_contents.clone().len());
    debug!("\n{}", stripped_contents.clone());

    // let mut lines: Vec<String> = contents
    //     .split("\n")
    //     .map(|s| String::from(s.trim()))
    //     .collect();

    // debug!("{:?}", lines);
    //
    // for line in lines.iter_mut() {
    //     line.make_ascii_uppercase();
    // }
    // debug!("{:?}", lines);

    // write_binary()?;

    Err("NOT IMPL".to_string())
}

fn write_binary(data: &WriteData) -> Result<String, String> {
    hfs::error(fs::create_dir_all(BINARY_PATH))?;

    let mut file = File::create(PathBuf::from(format!(
        "{}{}{}",
        BINARY_PATH,
        data.build_data.0,
        hta_shared::FILE_EXT_BINARY
    )))
    .expect("Could not create a binary file to write to!");

    hfs::error_u64(file.seek(SeekFrom::Start(0)))?;

    // HTA header
    writer::header(&mut file, "HTA")?;

    // Compiler Version
    writer::version(&mut file, data.compiler_version)?;

    // DBG
    match &data.debug_data {
        None => info!("No debug data is being written to the binary."),
        Some(dbg) => {
            writer::header(&mut file, "DBG")?;
            writer::debug_data(&mut file, dbg)?;
        }
    }

    // META
    writer::header(&mut file, "META")?;
    writer::metadata(&mut file, &data.metadata)?;

    // PROGRAM
    writer::header(&mut file, "PROGRAM")?;
    writer::program(&mut file, &data.program)?;

    // HTAEND
    writer::header(&mut file, "HTAEND")?;

    Err("NOT IMPL".to_string())
}

//TODO Own file?
mod file {
    use std::fs;

    pub fn intake(path: &str) -> Result<String, String> {
        match fs::read_to_string(path) {
            Ok(str) => Ok(str),
            Err(_) => Err(format!(
                "Failed to import file {}. (Compilation Failed)",
                path
            )),
        }
    }
}
