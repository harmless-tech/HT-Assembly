use crate::components::Type;

pub struct Instructions {
    index: u64,
    instructions: Vec<String>, //TODO Change
    arg1: Vec<String>
}

pub enum Instruction<T> {
    Alloc {
        name: String,
        hta_type: Type,
        default: Option<T>
    },
    SetVar {
        name: String,
        hta_type: Type,
        default: Option<T>
    }
}
