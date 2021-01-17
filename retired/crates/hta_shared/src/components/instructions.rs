use std::any::Any;

use crate::components::{Operations, Registers, Types};

//TODO Const strings for values.

#[derive(Debug)]
pub enum Instructions {
    Alloc {
        // A variables type will never change.
        name: String,
        hta_type: Types,
        default: Option<Box<dyn Any>>
    },
    SetVar {
        name: String,
        data: Box<dyn Any>
    },
    RegVar {
        name: String
    },
    SetReg {
        register: Registers,
        hta_type: Types,
        default: Option<Box<dyn Any>>
    },
    VarReg {
        name: String,
        register: Registers
    },
    CpyReg {
        register: Registers
    },
    Op {
        operation: Operations
    },
    PushJmp {
        //TODO Not very safe in milestone 1. Pushes are required to be popped manually.
        tag: String,
        name: Option<Box<dyn Any>>
    },
    PopJmp,
    Loop {
        tag: String,
        name: Option<Box<dyn Any>>
    },
    Cast {
        register: Registers,
        hta_type: Types
    },
    Native {
        native_name: String, //TODO Change to something else.
        args: Vec<Box<dyn Any>>
    },
    Return, //TODO Not in milestone1!!!
    Exit {
        code: i32
    },
    Blank // Used for tags.
}
/*impl Instructions {
    //TODO get function needs to be changed to account for args.
    /*fn get(val: &str) -> Option<Self> {
        match val.to_lowercase().as_str() {
            "alloc" => Option::Some(Instruction::Alloc {})
            _ => Option::None
        }
    }*/
}*/
