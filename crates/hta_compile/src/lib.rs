mod crafter;

use log::{debug, error, info, trace, warn};
use std::{fs, mem};
use std::fs::File;
use std::path::{PathBuf};
use std::io::{Seek, SeekFrom};
use std::convert::TryInto;

static DEFAULT_PATH: &str = "build/bin/";
static DEFAULT_FILE: &str = "main.hae";

// struct CompileData {}

// pub fn compile_with_export() {
//     //TODO
// }

//TODO Returned compiled struct.
//TODO Support multiple files.
pub fn compile(hta_file: &str) {
    /*let compiler_version = */match option_env!("CARGO_PKG_VERSION") {
        Some(v) => {
            info!("HTA Compiler Version: {}", v);
            v
        },
        None => {
            error!("No compiler version found defaulting to v0.0.1");
            "0.0.1"
        }
    };

    //TODO Remove!
    fs::create_dir_all(DEFAULT_PATH).unwrap();

    {
        use std::io::Write;
        // Header "HTA"
        let mut file = File::create(PathBuf::from(format!("{}{}", DEFAULT_PATH, DEFAULT_FILE))).unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();

        let buffer = &["HTA".as_bytes(), &[0_u8; 5]].concat();
        file.write_all(buffer).unwrap();

        // Write header info
        let header_buffer = &["HINFO".as_bytes(), &[0_u8; 3]].concat();
        let buffer = "Test, Harmless_Tech, 0.0.1".as_bytes();
        let chunk_size = &(buffer.len() as u64).to_be_bytes();
        file.write_all(header_buffer).unwrap();
        file.write_all(chunk_size).unwrap();
        file.write_all(buffer).unwrap();
    }

    {
        use std::io::Read;
        // Read back file
        let mut file = File::open(PathBuf::from(format!("{}{}", DEFAULT_PATH, DEFAULT_FILE))).unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();

        let mut buffer = [0_u8; 256];

        file.by_ref().take(8).read(&mut buffer).unwrap();
        assert_eq!(&buffer[0..3], "HTA".as_bytes());

        file.by_ref().take(8).read(&mut buffer).unwrap();
        assert_eq!(&buffer[0..5], "HINFO".as_bytes());

        file.by_ref().take(8).read(&mut buffer).unwrap();
        let (int_bytes, _) = buffer.split_at(mem::size_of::<u64>());
        assert_eq!(int_bytes, &(26 as u64).to_be_bytes());
        let chunk_size = u64::from_be_bytes(int_bytes.try_into().unwrap());
        let mut buffer =  vec![0_u8; chunk_size as usize];

        file.by_ref().take(chunk_size).read(&mut buffer).unwrap();
        assert_eq!(&buffer[0..(chunk_size as usize)], "Test, Harmless_Tech, 0.0.1".as_bytes());
    }
    //

    // let contents: String = intake_file(hta_file);
    // let lines: Vec<String> = contents
    //     .split("\n")
    //     .map(|s| String::from(s.trim()))
    //     .collect();
}

// #[inline]
// fn intake_file(path: &str) -> String {
//     match fs::read_to_string(path) {
//         Ok(str) => str,
//         Err(_) => {
//             error!("Failed to import file {}. (Compilation Failed)", path);
//             exit(-1);
//         }
//     }
// }
