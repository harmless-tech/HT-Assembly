pub mod version;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, iter::Map};

pub static FILE_EXT_CODE: &str = ".ha"; //
pub static FILE_EXT_BINARY: &str = ".hab"; //
                                           // pub static DEBUG_FILE_EXT: &str = ".hadbg"; This will be build into the binary file.
pub static FILE_EXT_SNAPSHOT: &str = ".hasnap"; //TODO Maybe?

/**
 * This struct holds the metadata of the HTA program.
 * This struct is created during compile time and then exported to the binary.
 * The runtime will then read in this data in during program startup.
 * This data is for the runtime only, the program cannot access it.
 */
#[derive(Clone, Debug)]
pub struct HTAMetaData {
    pub name: String,
    pub authors: Vec<String>,
    pub version: String,
    pub website: String,
    pub git: String,
    pub license: String,
    pub custom: Map<String, String>
}

/**
 * This struct holds mappings from the compiled program to the un-compiled program.
 * This is only generated for debug builds.
 */
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HTADebugData {
    pub call_function_mappings: HashMap<u64, String>, // Call Function Hash, Call Function Name
    pub namespace_mappings: HashMap<u64, String>,     // Namespace Hash, Namespace name
    pub variable_name_mappings: HashMap<u64, String>, // Namespace + Variable Hash, Namespace + Variable Name
    pub tag_name_mappings: HashMap<u64, String>,      // Namespace + Tag Hash, Namespace + Tag Name
    pub line_mappings: HashMap<(u64, u64), String> // Instruction Frame and Instruction Count, File Name + File Line Number
}
