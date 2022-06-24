pub mod before;

use hta_shared::old::lib::{Instructions, Tag, TagMap};
use std::collections::HashMap;

fn err_message<T>(file_name: &str, line_num: usize, message: &str) -> Result<T, String> {
    Err(format!(
        "File: {}, Line: {}: {}",
        file_name,
        line_num + 1,
        message
    ))
}

//TODO Compile.
pub fn compile() -> Result<(Vec<Instructions>, HashMap<Tag, TagMap>), String> {
    //let mut variables = HashMap::new(); // Hashmap to keep track of variable types.

    Ok((Vec::new(), HashMap::new()))
}

//TODO Linker Check.
// pub fn linker() {}

//TODO Safety Checks.
// pub fn safety_checks() {}

//TODO Optimize.
// pub fn optimize() {}
