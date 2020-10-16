use std::{any::Any, collections::HashMap, iter::Map};

use crate::components::{Instructions, Types};

//TODO This will hold all of the information that is needed by the runtime.
#[derive(Debug)]
pub struct HTADatabase {
    pub entry_frame: String,
    pub frames: HashMap<String, HTAFrame>
}

#[derive(Debug)]
pub struct HTAFrame {
    pub instructions: Vec<Instructions>,
    pub tags: HashMap<String, u32>,
    pub vars: HashMap<String, (Types, Box<dyn Any>)> // When exit is hit all of these should be thrown away.
}
