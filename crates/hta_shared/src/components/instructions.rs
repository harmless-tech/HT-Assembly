use crate::components::Type;

pub struct Instructions {
    index: u64,
    instructions: Vec<String>, //TODO Change
    arg1: Vec<String>
}

pub enum Instruction<T> {
    Alloc { // A variables type will never change.
        name: String,
        hta_type: Type,
        default: Option<T>
    },
    SetVar {
        name: String,
        data: T
    },
    RegVar {
        name: String
    }
}
