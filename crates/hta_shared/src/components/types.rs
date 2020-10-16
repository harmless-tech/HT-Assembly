use std::mem;

//TODO Const string for values.

#[derive(Debug)]
pub enum Types {
    Char = 0x0000,   // chr
    String = 0x0001, // str
    Int8 = 0x0002,   // i8
    Int16 = 0x0003,  // i16
    Int32 = 0x0004,  // i32
    Int64 = 0x0005,  // i64
    Int128 = 0x0006, // i128
    //Int = 0x0007,     // int, Equal to Int32 //TODO i64??
    UInt8 = 0x0008,   // u8
    UInt16 = 0x0009,  // u16
    UInt32 = 0x000a,  // u32
    UInt64 = 0x000b,  // u64
    UInt128 = 0x000c, // u128
    //UInt = 0x000d,    // uint, Equal to UInt32 //TODO u64??
    Float32 = 0x000e, // f32
    Float64 = 0x000f, // f64
    //Float = 0x0010,   // float, Equal to Float64
    //Byte = 0x0020,    // byte, Equal to Uint8
    Boolean = 0x0030, // bool
    Object = 0x0040,  // obj
    Void = 0x0050     // void
                      //TODO Pointer type?? Arrays??
}
impl Types {
    pub fn get(val: &str) -> Option<Self> {
        match val.to_lowercase().as_str() {
            "chr" => Option::Some(Types::Char),
            "str" => Option::Some(Types::String),
            "i8" => Option::Some(Types::Int8),
            "i16" => Option::Some(Types::Int16),
            "i32" | "int" => Option::Some(Types::Int32),
            "i64" => Option::Some(Types::Int64),
            "i128" => Option::Some(Types::Int128),
            "u8" | "byte" => Option::Some(Types::UInt8),
            "u16" => Option::Some(Types::UInt16),
            "u32" | "uint" => Option::Some(Types::UInt32),
            "u64" => Option::Some(Types::UInt64),
            "u128" => Option::Some(Types::UInt128),
            "f32" => Option::Some(Types::Float32),
            "f64" | "float" => Option::Some(Types::Float64),
            "bool" => Option::Some(Types::Boolean),
            "obj" => Option::Some(Types::Object),
            "void" => Option::Some(Types::Void),
            _ => Option::None
        }
    }

    fn value(&self) -> String {
        match self {
            Types::Char => String::from("chr"),
            Types::String => String::from("str"),
            Types::Int8 => String::from("i8"),
            Types::Int16 => String::from("i16"),
            Types::Int32 => String::from("i32"),
            Types::Int64 => String::from("i64"),
            Types::Int128 => String::from("i128"),
            //Type::Int => String::from("int"),
            Types::UInt8 => String::from("u8"),
            Types::UInt16 => String::from("u16"),
            Types::UInt32 => String::from("u32"),
            Types::UInt64 => String::from("u64"),
            Types::UInt128 => String::from("u128"),
            //Type::UInt => String::from("uint"),
            Types::Float32 => String::from("f32"),
            Types::Float64 => String::from("f64"),
            //Type::Float => String::from("float"),
            //Type::Byte => String::from("byte"),
            Types::Boolean => String::from("bool"),
            Types::Object => String::from("obj"),
            Types::Void => String::from("void")
        }
    }
}

#[derive(Debug)]
pub enum Registers {
    R0 = 0x0000,
    R1 = 0x0001 //RReturn = 0x0002 - Return register is readonly anyways, so it does not need a value.
}
impl Registers {
    pub fn get(val: &str) -> Option<Self> {
        match val.to_lowercase().as_str() {
            "r0" | "0" => Option::Some(Registers::R0),
            "r1" | "1" => Option::Some(Registers::R1),
            _ => Option::None
        }
    }

    fn value(&self) -> String {
        match self {
            Registers::R0 => String::from("r0"),
            Registers::R1 => String::from("r1") //Registers::RReturn => String::from("rreturn")
        }
    }
}

#[derive(Debug)]
pub enum Operations {
    // Logic
    Not = 0x0000, // !
    And = 0x0001, // &&
    Or = 0x0002,  // ||
    Xor = 0x0003, // ^ (May become bitwise after milestone 1)
    // Arith
    Mod = 0x0004,   // %
    Mul = 0x0005,   // *
    Div = 0x0006,   // /
    Plus = 0x0007,  // +
    Minus = 0x0008, // -
    // Rel
    Equal = 0x0009,        // ==
    NotEqual = 0x000a,     // !=
    Greater = 0x000b,      // >
    Less = 0x000c,         // <
    GreaterEqual = 0x000d, // >=
    LessEqual = 0x0000e    // <=
                           // Bitwise TODO Bitwise ops.
}
impl Operations {
    pub fn get(val: &str) -> Option<Self> {
        match val.to_lowercase().as_str() {
            "!" => Option::Some(Operations::Not),
            "&&" => Option::Some(Operations::And),
            "||" => Option::Some(Operations::Or),
            "^" => Option::Some(Operations::Xor),
            "%" => Option::Some(Operations::Mod),
            "*" => Option::Some(Operations::Mul),
            "/" => Option::Some(Operations::Div),
            "+" => Option::Some(Operations::Plus),
            "-" => Option::Some(Operations::Minus),
            "==" => Option::Some(Operations::Equal),
            "!=" => Option::Some(Operations::NotEqual),
            ">" => Option::Some(Operations::Greater),
            "<" => Option::Some(Operations::Less),
            ">=" => Option::Some(Operations::GreaterEqual),
            "<=" => Option::Some(Operations::LessEqual),
            _ => Option::None
        }
    }

    fn value(&self) -> String {
        match self {
            Operations::Not => String::from("!"),
            Operations::And => String::from("&&"),
            Operations::Or => String::from("||"),
            Operations::Xor => String::from("^"),
            Operations::Mod => String::from("%"),
            Operations::Mul => String::from("*"),
            Operations::Div => String::from("/"),
            Operations::Plus => String::from("+"),
            Operations::Minus => String::from("-"),
            Operations::Equal => String::from("=="),
            Operations::NotEqual => String::from("!="),
            Operations::Greater => String::from(">"),
            Operations::Less => String::from("<"),
            Operations::GreaterEqual => String::from(">="),
            Operations::LessEqual => String::from("<=")
        }
    }
}
