use std::{fmt, fmt::Formatter, mem};

#[derive(Debug)]
pub enum Type {
    Char = 0x0000,    // chr
    String = 0x0001,  // str
    Int8 = 0x0002,    // i8
    Int16 = 0x0003,   // i16
    Int32 = 0x0004,   // i32
    Int64 = 0x0005,   // i64
    Int = 0x0006,     // int, Equal to Int32
    UInt8 = 0x0007,   // u8
    UInt16 = 0x0008,  // u16
    UInt32 = 0x0009,  // u32
    UInt64 = 0x000a,  // u64
    UInt = 0x000b,    // uint, Equal to UInt32
    Float32 = 0x000c, // f32
    Float64 = 0x000d, // f64
    Float = 0x000e,   // float, Equal to Float64
    Byte = 0x000f,    // byte, Equal to Uint8
    Boolean = 0x0010, // bool
    Object = 0x0020,  // obj
    Void = 0x0030     // void
                      //TODO Pointer type?? Arrays??
}
impl fmt::Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Type {
    fn value(&self) -> String {
        match self {
            Type::Char => String::from("chr"),
            Type::String => String::from("str"),
            Type::Int8 => String::from("i8"),
            Type::Int16 => String::from("i16"),
            Type::Int32 => String::from("i32"),
            Type::Int64 => String::from("i64"),
            Type::Int => String::from("int"),
            Type::UInt8 => String::from("u8"),
            Type::UInt16 => String::from("u16"),
            Type::UInt32 => String::from("u32"),
            Type::UInt64 => String::from("u64"),
            Type::UInt => String::from("uint"),
            Type::Float32 => String::from("f32"),
            Type::Float64 => String::from("f64"),
            Type::Float => String::from("float"),
            Type::Byte => String::from("byte"),
            Type::Boolean => String::from("bool"),
            Type::Object => String::from("obj"),
            Type::Void => String::from("void")
        }
    }
}
