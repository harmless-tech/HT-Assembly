use crate::components::Type;

pub struct Instructions {
    index: u64,
    instructions: Vec<String>, //TODO Change
    arg1: Vec<String>
}

pub enum Instruction {
    Alloc {
        name: String,
        hta_type: Type,
        default: Option<()>
    },
    SetVar {
        name: String,
        hta_type: Type,
        default: Option<()>
    }
}
