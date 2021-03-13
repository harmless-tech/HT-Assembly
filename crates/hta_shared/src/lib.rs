use std::iter::Map;

/**
 * This struct holds the metadata of the HTA program.
 * This struct is created during compile time and then exported to the binary.
 * The runtime will then read in this data in during program startup.
 * This data is for the runtime only, the program cannot access it.
 */
pub struct HTAMetaData {
    pub name: String,
    pub authors: Vec<String>,
    pub version: String,
    pub website: String,
    pub git: String,
    pub license: String,
    pub custom: Map<String, String>
}
