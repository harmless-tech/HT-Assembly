pub mod hfs;
pub mod traits;
pub mod version;

use log::error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub static FILE_EXT_CODE: &str = ".ha"; // File extension for code files.
pub static FILE_EXT_BINARY: &str = ".hab"; // File extension for built binary files.
pub static DEBUG_FILE_EXT: &str = ".hadbg"; // This will be build into the binary file. TODO Maybe?
pub static FILE_EXT_SNAPSHOT: &str = ".hasnap"; //TODO Maybe?

pub type Tag = u64;
pub type TagMap = (u64, u64); // Frame, Instruction
pub type Variable = u64;
pub type NativeName = u64; // This is for the name of the native library and the native function being called

#[derive(Clone, Debug, PartialEq)]
pub enum Instructions {
    Alloc(Variable, DataType),  // Variable, DataType, Optional<Data>
    SetVar(Variable, DataType), // Variable, Data
    RegVar(Variable),           // Variable
    SetReg(Register, DataType), // Register, DataType, Optional<Data>
    VarReg(Variable, Register), // Variable, Register
    CpyReg(Register, Register), // Register, Register
    Op(Operation),              // Operation, Optional<u64>
    Jmp(Tag),                   // Tag
    PushJmp(Tag),               // Tag
    PopJmp,                     //
    Cast(DataType),             // DataType
    Call(NativeName),           // NativeName
    Exit(i32),                  // i32
    Assert(Option<DataType>),   // Optional<DataType<Data>>
}
impl traits::EnumWithU8 for Instructions {
    fn to_u8(&self) -> u8 {
        match self {
            Instructions::Alloc(_, _) => 0,
            Instructions::SetVar(_, _) => 1,
            Instructions::RegVar(_) => 2,
            Instructions::SetReg(_, _) => 3,
            Instructions::VarReg(_, _) => 4,
            Instructions::CpyReg(_, _) => 5,
            Instructions::Op(_) => 6,
            Instructions::Jmp(_) => 7,
            Instructions::PushJmp(_) => 8,
            Instructions::PopJmp => 9,
            Instructions::Cast(_) => 10,
            Instructions::Call(_) => 11,
            Instructions::Exit(_) => 12,
            Instructions::Assert(_) => 13,
        }
    }

    fn from_u8(e: u8) -> Self {
        match e {
            0 => Instructions::Alloc(0, DataType::Char(0 as char)),
            1 => Instructions::SetVar(0, DataType::Char(0 as char)),
            2 => Instructions::RegVar(0),
            3 => Instructions::SetReg(Register::C0, DataType::Char(0 as char)),
            4 => Instructions::VarReg(0, Register::C0),
            5 => Instructions::CpyReg(Register::C0, Register::C0),
            6 => Instructions::Op(Operation::Logic(LogicOperation::Not)),
            7 => Instructions::Jmp(0),
            8 => Instructions::PushJmp(0),
            9 => Instructions::PopJmp,
            10 => Instructions::Cast(DataType::Char(0 as char)),
            11 => Instructions::Call(0),
            12 => Instructions::Exit(0),
            13 => Instructions::Assert(None),
            _ => {
                error!("Instruction not found, defaulting to Instruction Alloc!");
                Instructions::Alloc(0, DataType::Char(0 as char))
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Register {
    C0 = 0x0000, // Comp 1, $0
    C1 = 0x0001, // Comp 2, $1
    C2 = 0x0010, // Return, $2
}

#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    Char(char),     // chr, Default: 0
    String(String), // str, Default: ""
    Int8(i8),       // i8, Default: 0
    Int16(i16),     // i16, Default: 0
    Int32(i32),     // Int, i32, Default: 0
    Int64(i64),     // i64, Default: 0
    Int128(i128),   // i128, Default: 0
    UInt8(u8),      // u8, Default: 0
    UInt16(u16),    // u16, Default: 0
    UInt32(u32),    // UInt, u32, Default: 0
    UInt64(u64),    // u64, Default: 0
    UInt128(u128),  // u128, Default: 0
    Float32(f32),   // f32, Default: 0.0
    Float64(f64),   // Float, f64, Default: 0.0
    Boolean(bool),  // bool, Default: false
}
impl traits::EnumWithU8 for DataType {
    fn to_u8(&self) -> u8 {
        match self {
            DataType::Char(_) => 0,
            DataType::String(_) => 1,
            DataType::Int8(_) => 2,
            DataType::Int16(_) => 3,
            DataType::Int32(_) => 4,
            DataType::Int64(_) => 5,
            DataType::Int128(_) => 6,
            DataType::UInt8(_) => 7,
            DataType::UInt16(_) => 8,
            DataType::UInt32(_) => 9,
            DataType::UInt64(_) => 10,
            DataType::UInt128(_) => 11,
            DataType::Float32(_) => 12,
            DataType::Float64(_) => 13,
            DataType::Boolean(_) => 14,
        }
    }

    fn from_u8(e: u8) -> Self {
        match e {
            0 => DataType::Char(0 as char),
            1 => DataType::String(String::new()),
            2 => DataType::Int8(0),
            3 => DataType::Int16(0),
            4 => DataType::Int32(0),
            5 => DataType::Int64(0),
            6 => DataType::Int128(0),
            7 => DataType::UInt8(0),
            8 => DataType::UInt16(0),
            9 => DataType::UInt32(0),
            10 => DataType::UInt64(0),
            11 => DataType::UInt128(0),
            12 => DataType::Float32(0.0),
            13 => DataType::Float64(0.0),
            14 => DataType::Boolean(false),
            _ => {
                error!("DataType not found, defaulting to DataType Char!");
                DataType::Char(0 as char)
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    Logic(LogicOperation),
    Arithmetic(ArithmeticOperation),
    Relational(RelationalOperation),
    BitWise(BitWiseOperation), //TODO Not in m1!
}
impl traits::EnumWithU8 for Operation {
    fn to_u8(&self) -> u8 {
        match self {
            Operation::Logic(l) => match l {
                LogicOperation::Not => 0,
                LogicOperation::And => 1,
                LogicOperation::Or => 2,
            },
            Operation::Arithmetic(a) => match a {
                ArithmeticOperation::Add => 3,
                ArithmeticOperation::Subtract => 4,
                ArithmeticOperation::Multiplication => 5,
                ArithmeticOperation::Division => 6,
                ArithmeticOperation::Modulus => 7,
                ArithmeticOperation::Increment => 8,
                ArithmeticOperation::Decrement => 9,
            },
            Operation::Relational(r) => match r {
                RelationalOperation::Equal => 10,
                RelationalOperation::NotEqual => 11,
                RelationalOperation::Greater => 12,
                RelationalOperation::Less => 13,
                RelationalOperation::GreaterEqual => 14,
                RelationalOperation::LessEqual => 15,
            },
            Operation::BitWise(b) => match b {
                BitWiseOperation::And => 16,
                BitWiseOperation::Or => 17,
                BitWiseOperation::Xor => 18,
                BitWiseOperation::Not => 19,
                BitWiseOperation::ShiftLeft(_) => 20,
                BitWiseOperation::ShiftRight(_) => 21,
            },
        }
    }

    fn from_u8(e: u8) -> Self {
        match e {
            0 => Operation::Logic(LogicOperation::Not),
            1 => Operation::Logic(LogicOperation::And),
            2 => Operation::Logic(LogicOperation::Or),

            3 => Operation::Arithmetic(ArithmeticOperation::Add),
            4 => Operation::Arithmetic(ArithmeticOperation::Subtract),
            5 => Operation::Arithmetic(ArithmeticOperation::Multiplication),
            6 => Operation::Arithmetic(ArithmeticOperation::Division),
            7 => Operation::Arithmetic(ArithmeticOperation::Modulus),
            8 => Operation::Arithmetic(ArithmeticOperation::Increment),
            9 => Operation::Arithmetic(ArithmeticOperation::Decrement),

            10 => Operation::Relational(RelationalOperation::Equal),
            11 => Operation::Relational(RelationalOperation::NotEqual),
            12 => Operation::Relational(RelationalOperation::Greater),
            13 => Operation::Relational(RelationalOperation::Less),
            14 => Operation::Relational(RelationalOperation::GreaterEqual),
            15 => Operation::Relational(RelationalOperation::LessEqual),

            16 => Operation::BitWise(BitWiseOperation::And),
            17 => Operation::BitWise(BitWiseOperation::Or),
            18 => Operation::BitWise(BitWiseOperation::Xor),
            19 => Operation::BitWise(BitWiseOperation::Not),
            20 => Operation::BitWise(BitWiseOperation::ShiftLeft(0)),
            21 => Operation::BitWise(BitWiseOperation::ShiftRight(0)),

            _ => {
                error!("Operation not found, defaulting to Logic Not!");
                Operation::Logic(LogicOperation::Not)
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LogicOperation {
    Not, // !
    And, // &&
    Or,  // ||
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArithmeticOperation {
    Add,            // +
    Subtract,       // -
    Multiplication, // *
    Division,       // /
    Modulus,        // %
    Increment,      // ++
    Decrement,      // --
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RelationalOperation {
    Equal,        // ==
    NotEqual,     // !=
    Greater,      // >
    Less,         // <
    GreaterEqual, //>=
    LessEqual,    // <=
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BitWiseOperation {
    And,             // &
    Or,              // |
    Xor,             // ^
    Not,             // ~
    ShiftLeft(u64),  // << u64
    ShiftRight(u64), // >> u64
}

/**
 * This struct holds mappings from the compiled program to the un-compiled program.
 * This is only generated for debug builds.
 */
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DebugData {
    pub native_lib_mappings: HashMap<NativeName, String>, // Library Hash, Library Name
    pub call_function_mappings: HashMap<NativeName, String>, // Call Function Hash, Call Function Name
    pub namespace_mappings: HashMap<u64, String>, // Namespace Hash, Namespace name //TODO Needed?
    pub variable_name_mappings: HashMap<Variable, String>, // Namespace + Variable Hash, Namespace + Variable Name
    pub tag_name_mappings: HashMap<Tag, String>, // Namespace + Tag Hash, Namespace + Tag Name
    pub line_mappings: HashMap<TagMap, String>, // Instruction Frame and Instruction Count, File Name + File Line Number
}

/**
 * This struct holds the metadata of the HTA program.
 * This struct is created during compile time and then exported to the binary.
 * The runtime will then read in this data in during program startup.
 * This data is for the runtime only, the program cannot access it.
 */
#[derive(Clone, Debug)]
pub struct MetaData {
    pub name: String,
    pub authors: Vec<String>,
    pub version: String,
    pub website: String,
    pub git: String,
    pub license: String,
    pub natives: Vec<NativeName>, // Required native libraries
                                  // pub custom: Map<String, String> //TODO Add this later. Maybe?
}

#[derive(Clone, Debug)]
pub struct Program {
    pub tags: HashMap<Tag, TagMap>,
    pub instructions: Vec<Vec<Instructions>>,
}
