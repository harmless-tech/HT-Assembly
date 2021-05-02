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
static DEFAULT_BINARY_NAME: &str = "main";

#[derive(Debug)]
pub struct WriteData {
    pub build_data: (String, String), // Right now this is just a file name. (File Name, EMPTY)
    pub compiler_version: (u64, u64, u64),
    // pub debug_data: Option<DebugData>, This will be passed separately.
    pub metadata: MetaData,
    // pub program: Program, This will be passed separately.
}
impl WriteData {
    fn new(compiler_version: (u64, u64, u64)) -> Self {
        WriteData {
            build_data: (DEFAULT_BINARY_NAME.clone().to_string(), "".to_string()),
            compiler_version,
            metadata: MetaData {
                name: "".to_string(),
                authors: vec![],
                version: "".to_string(),
                website: "".to_string(),
                git: "".to_string(),
                license: "".to_string(),
                natives: vec!["std".to_string()]
            }
        }
    }
}

// Returns the binary file name on success.
//TODO Allow for multiple errors to be returned.
pub fn compile(hta_file: &str, hta_file_name: &str, dbg: bool) -> Result<String, String> {
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

    // Process the entry file.
    let contents = file::intake(hta_file)?; // Import the entry file.
    let contents = compiler::remove_comments(hta_file_name, contents)?; // Strip comments from entry file.

    debug!("Size: {}", contents.clone().len());
    debug!("REMOVE COMMENTS: \n{}", contents.clone());

    let mut lines: Vec<String> = contents
        .split("\n")
        .map(|s| String::from(s.trim()))
        .collect();

    let mut write_data = WriteData::new(compiler_version);
    compiler::pre_process_entry(&mut write_data, hta_file_name, &mut lines)?;

    debug!("REMOVE ENTRY PRE STATEMENTS: \n{}", lines.clone().join("\n"));
    debug!("WRITE DATA: {:?}", write_data);

    // debug!("{:?}", lines);
    //
    // for line in lines.iter_mut() {
    //     line.make_ascii_uppercase();
    // }
    // debug!("{:?}", lines);

    // write_binary()?;

    Err("NOT IMPL".to_string())
}

fn write_binary(data: &WriteData, debug_data: &Option<DebugData>, program: &Program) -> Result<String, String> {
    hfs::error(fs::create_dir_all(BINARY_PATH))?;

    let path = format!(
        "{}{}{}",
        BINARY_PATH,
        data.build_data.0,
        hta_shared::FILE_EXT_BINARY
    );
    let mut file = File::create(PathBuf::from(path.clone()))
        .expect("Could not create a binary file to write to!");

    hfs::error_u64(file.seek(SeekFrom::Start(0)))?;

    // HTA header
    writer::header(&mut file, "HTA")?;

    // Compiler Version
    writer::version(&mut file, data.compiler_version)?;

    // DBG
    match &debug_data {
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
    writer::program(&mut file, &program)?;

    // HTAEND
    writer::header(&mut file, "HTAEND")?;

    Ok(format!("Binary written to {}.", path))
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
