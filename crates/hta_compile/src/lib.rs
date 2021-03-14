mod crafter;
mod writer;

use hta_shared::{version::parse_version_str, DebugData, Instructions, MetaData, Program};
use log::{debug, error, info, trace, warn};
use std::{
    convert::TryInto,
    fs,
    fs::File,
    io::{Seek, SeekFrom},
    mem,
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
    build_data: (String), // Right now this is just a file name.
    compiler_version: (u64, u64, u64),
    debug_data: DebugData,
    metadata: MetaData,
    program: Program,
}

pub fn compile(hta_file: &str, dbg: bool) -> Result<(), String> {
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

    // let contents: String = intake_file(hta_file);
    // let lines: Vec<String> = contents
    //     .split("\n")
    //     .map(|s| String::from(s.trim()))
    //     .collect();

    Err("NOT IMPL".to_string())
}

/*
 * All header are 8 bytes.
 * All sizes are u64, unless otherwise stated.
 *
 * HTA Header
 *     Compiler Version
 *         Major
 *         Minor
 *         Patch
 * DBG Header
 *     Debug Data
 *         Size of struct
 *         Struct using bincode
 * META Header
 *     Repeat for each piece of data
 *         Size
 *         Info
 *     Amount of Natives
 *     Repeat
 *         Native Name
 * PROGRAM header
 *     Tag Map
 *         Amount of Tags
 *         Repeat
 *             Tag
 *             Frame
 *             Instruction location
 *     Instructions
 *         Amount of Frames
 *         Repeat
 *             Amount of Instructions
 *                 Repeat
 *                 Instruction (u8)
 *                 Data for instruction (with sizing when needed)
 * END header
 *     Any data can be put here, it will be ignored by the runtime.
 */
fn write_binary(data: &WriteData) -> Result<(), String> {
    Instructions::Exit(-1);

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

/*
* Example on how to create the binary file.
*
   fs_error(fs::create_dir_all(BINARY_PATH))?;

   {
       use std::io::Write;
       // Header "HTA"
       let mut file = File::create(PathBuf::from(format!(
           "{}{}{}",
           BINARY_PATH,
           DEFAULT_FILE_NAME,
           hta_shared::FILE_EXT_BINARY
       )))
       .unwrap();
       fs_error_u64(file.seek(SeekFrom::Start(0)))?;

       let buffer = &["HTA".as_bytes(), &[0_u8; 5]].concat();
       fs_error(file.write_all(buffer))?;

       // Write header info
       let header_buffer = &["HINFO".as_bytes(), &[0_u8; 3]].concat();
       let buffer = "Test, Harmless_Tech, 0.0.1".as_bytes();
       let chunk_size = &(buffer.len() as u64).to_be_bytes();
       fs_error(file.write_all(header_buffer))?;
       fs_error(file.write_all(chunk_size))?;
       fs_error(file.write_all(buffer))?;

       // Write debug struct
       let debug_info = hta_shared::HTADebugData {
           call_function_mappings: Default::default(),
           namespace_mappings: Default::default(),
           variable_name_mappings: Default::default(),
           tag_name_mappings: Default::default(),
           line_mappings: Default::default()
       };

       let header_buffer = &["DBG".as_bytes(), &[0_u8; 5]].concat();
       let buffer = bincode::serialize(&debug_info).unwrap();
       let chunk_size = &(buffer.len() as u64).to_be_bytes();
       fs_error(file.write_all(header_buffer))?;
       fs_error(file.write_all(chunk_size))?;
       fs_error(file.write_all(&buffer))?;

       // Program
       let header_buffer = &["PROGRAM".as_bytes(), &[0_u8; 1]].concat();
       //let buffer = bincode::serialize(&debug_info).unwrap();
       //let chunk_size = &(buffer.len() as u64).to_be_bytes();
       fs_error(file.write_all(header_buffer))?;
       //file.write_all(chunk_size).unwrap();
       //file.write_all(&buffer).unwrap();

       // End
       let header_buffer = &["END".as_bytes(), &[0_u8; 5]].concat();
       fs_error(file.write_all(header_buffer))?;
   }

   {
       use std::io::Read;
       // Read back file
       let mut file = File::open(PathBuf::from(format!(
           "{}{}{}",
           BINARY_PATH,
           DEFAULT_FILE_NAME,
           hta_shared::FILE_EXT_BINARY
       )))
       .unwrap();
       fs_error_u64(file.seek(SeekFrom::Start(0)))?;

       let mut buffer = [0_u8; 8];

       fs_error_usize(file.by_ref().take(8).read(&mut buffer))?;
       assert_eq!(&buffer[0..3], "HTA".as_bytes());

       fs_error_usize(file.by_ref().take(8).read(&mut buffer))?;
       assert_eq!(&buffer[0..5], "HINFO".as_bytes());

       // Header info
       fs_error_usize(file.by_ref().take(8).read(&mut buffer))?;
       let (int_bytes, _) = buffer.split_at(mem::size_of::<u64>());
       assert_eq!(int_bytes, &(26 as u64).to_be_bytes());

       let chunk_size = u64::from_be_bytes(int_bytes.try_into().unwrap());
       let mut buffer = vec![0_u8; chunk_size as usize];

       fs_error_usize(file.by_ref().take(chunk_size).read(&mut buffer))?;
       assert_eq!(
           &buffer[0..(chunk_size as usize)],
           "Test, Harmless_Tech, 0.0.1".as_bytes()
       );

       // Debug
       let mut buffer = [0_u8; 8];

       fs_error_usize(file.by_ref().take(8).read(&mut buffer))?;
       assert_eq!(&buffer[0..3], "DBG".as_bytes());

       fs_error_usize(file.by_ref().take(8).read(&mut buffer))?;
       let (int_bytes, _) = buffer.split_at(mem::size_of::<u64>());

       let chunk_size = u64::from_be_bytes(int_bytes.try_into().unwrap());
       debug!("DBG Chunk Size: {}", chunk_size);
       let mut buffer = vec![0_u8; chunk_size as usize];

       fs_error_usize(file.by_ref().take(chunk_size).read(&mut buffer))?;

       // Program
       let mut buffer = [0_u8; 8];

       fs_error_usize(file.by_ref().take(8).read(&mut buffer))?;
       assert_eq!(&buffer[0..7], "PROGRAM".as_bytes());

       // End
       let mut buffer = [0_u8; 8];

       fs_error_usize(file.by_ref().take(8).read(&mut buffer))?;
       assert_eq!(&buffer[0..3], "END".as_bytes())
   }
   //
*/
