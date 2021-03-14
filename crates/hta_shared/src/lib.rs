pub mod hfs;
pub mod version;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, iter::Map};

pub static FILE_EXT_CODE: &str = ".ha"; // File extension for code files.
pub static FILE_EXT_BINARY: &str = ".hab"; // File extension for built binary files.
pub static DEBUG_FILE_EXT: &str = ".hadbg"; // This will be build into the binary file. TODO Maybe?
pub static FILE_EXT_SNAPSHOT: &str = ".hasnap"; //TODO Maybe?

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
    pub license: String //pub custom: Map<String, String> //TODO Add this later. Maybe?
}

/**
 * This struct holds mappings from the compiled program to the un-compiled program.
 * This is only generated for debug builds.
 */
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DebugData {
    pub call_function_mappings: HashMap<u64, String>, // Call Function Hash, Call Function Name
    pub namespace_mappings: HashMap<u64, String>,     // Namespace Hash, Namespace name
    pub variable_name_mappings: HashMap<u64, String>, // Namespace + Variable Hash, Namespace + Variable Name
    pub tag_name_mappings: HashMap<u64, String>,      // Namespace + Tag Hash, Namespace + Tag Name
    pub line_mappings: HashMap<(u64, u64), String>    // Instruction Frame and Instruction Count, File Name + File Line Number
}

#[derive(Clone, Debug)]
pub struct Program {
}

#[derive(Clone, Debug, PartialEq)]
pub enum Instructions {
    Alloc((Variable, DataType)),  // Variable, DataType, Optional<Data>
    SetVar((Variable, DataType)), // Variable, Data
    RegVar(Variable),             // Variable
    SetReg((Register, DataType)), // Register, DataType, Optional<Data>
    VarReg((Variable, Register)), // Variable, Register
    CpyReg((Register, Register)), // Register, Register
    Op(Operation),                // Operation, Optional<u64>
    Jmp(Tag),                     // Tag
    PushJmp(Tag),                 // Tag
    PopJmp,                       //
    Cast(DataType),               // DataType
    Call(NativeName),             // NativeName
    Exit(i32),                    // i32
    Assert(Option<DataType>)      // Optional<DataType<Data>>
}

type Tag = u64;
type Variable = u64;
type NativeName = u64;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Register {
    C0, // Comp 1, $0
    C1, // Comp 2, $1
    C2  // Return, $2
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
    Boolean(bool)   // bool, Default: false
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    Logic(LogicOperation),
    Arithmetic(ArithmeticOperation),
    Relational(RelationalOperation),
    BitWise(BitWiseOperation) //TODO Not in m1!
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LogicOperation {
    Not, // !
    And, // &&
    Or   // ||
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArithmeticOperation {
    Add,            // +
    Subtract,       // -
    Multiplication, // *
    Division,       // /
    Modulus,        // %
    Increment,      // ++
    Decrement       // --
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RelationalOperation {
    Equal,        // ==
    NotEqual,     // !=
    Greater,      // >
    Less,         // <
    GreaterEqual, //>=
    LessEqual     // <=
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BitWiseOperation {
    And,             // &
    Or,              // |
    Xor,             // ^
    Not,             // ~
    ShiftLeft(u64),  // << u64
    ShiftRight(u64)  // >> u64
}
