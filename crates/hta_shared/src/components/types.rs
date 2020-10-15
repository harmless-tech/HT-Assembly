use std::mem;

pub enum Type {
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
impl Type {
    fn get(val: &str) -> Option<Self> {
        match val.to_lowercase().as_str() {
            "chr" => Option::Some(Type::Char),
            "str" => Option::Some(Type::String),
            "i8" => Option::Some(Type::Int8),
            "i16" => Option::Some(Type::Int16),
            "i32" | "int" => Option::Some(Type::Int32),
            "i64" => Option::Some(Type::Int64),
            "i128" => Option::Some(Type::Int128),
            "u8" | "byte" => Option::Some(Type::UInt8),
            "u16" => Option::Some(Type::UInt16),
            "u32" | "uint" => Option::Some(Type::UInt32),
            "u64" => Option::Some(Type::UInt64),
            "u128" => Option::Some(Type::UInt128),
            "f32" => Option::Some(Type::Float32),
            "f64" | "float" => Option::Some(Type::Float64),
            "bool" => Option::Some(Type::Boolean),
            "obj" => Option::Some(Type::Object),
            "void" => Option::Some(Type::Void),
            _ => Option::None
        }
    }

    fn value(&self) -> String {
        match self {
            Type::Char => String::from("chr"),
            Type::String => String::from("str"),
            Type::Int8 => String::from("i8"),
            Type::Int16 => String::from("i16"),
            Type::Int32 => String::from("i32"),
            Type::Int64 => String::from("i64"),
            Type::Int128 => String::from("i128"),
            //Type::Int => String::from("int"),
            Type::UInt8 => String::from("u8"),
            Type::UInt16 => String::from("u16"),
            Type::UInt32 => String::from("u32"),
            Type::UInt64 => String::from("u64"),
            Type::UInt128 => String::from("u128"),
            //Type::UInt => String::from("uint"),
            Type::Float32 => String::from("f32"),
            Type::Float64 => String::from("f64"),
            //Type::Float => String::from("float"),
            //Type::Byte => String::from("byte"),
            Type::Boolean => String::from("bool"),
            Type::Object => String::from("obj"),
            Type::Void => String::from("void")
        }
    }
}

pub enum Registers {
    R0 = 0x0000,
    R1 = 0x0001
    //RReturn = 0x0002 - Return register is readonly anyways, so it does not need a value.
}
impl Registers {
    fn get(val: &str) -> Option<Self> {
        match val.to_lowercase().as_str() {
            "r0" | "0" => Option::Some(Registers::R0),
            "r1" | "1" => Option::Some(Registers::R1),
            _ => Option::None
        }
    }

    fn value(&self) -> String {
        match self {
            Registers::R0 => String::from("r0"),
            Registers::R1 => String::from("r1")
            //Registers::RReturn => String::from("rreturn")
        }
    }
}

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
    fn get(val: &str) -> Option<Self> {
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
