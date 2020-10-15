use std::mem;

pub enum Type {
    Char = 0x0000,    // chr
    String = 0x0001,  // str
    Int8 = 0x0002,    // i8
    Int16 = 0x0003,   // i16
    Int32 = 0x0004,   // i32
    Int64 = 0x0005,   // i64
    Int128 = 0x0006,  // i128
    Int = 0x0007,     // int, Equal to Int32 //TODO i64??
    UInt8 = 0x0008,   // u8
    UInt16 = 0x0009,  // u16
    UInt32 = 0x000a,  // u32
    UInt64 = 0x000b,  // u64
    UInt128 = 0x000c, // u128
    UInt = 0x000d,    // uint, Equal to UInt32 //TODO u64??
    Float32 = 0x000e, // f32
    Float64 = 0x000f, // f64
    Float = 0x0010,   // float, Equal to Float64
    Byte = 0x0020,    // byte, Equal to Uint8
    Boolean = 0x0030, // bool
    Object = 0x0040,  // obj
    Void = 0x0050     // void
                      //TODO Pointer type?? Arrays??
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
            Type::Int128 => String::from("i128"),
            Type::Int => String::from("int"),
            Type::UInt8 => String::from("u8"),
            Type::UInt16 => String::from("u16"),
            Type::UInt32 => String::from("u32"),
            Type::UInt64 => String::from("u64"),
            Type::UInt128 => String::from("u128"),
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

pub enum Registers {
    R0 = 0x0000,
    R1 = 0x0001,
    RReturn = 0x0002
}
impl Registers {
    fn value(&self) -> String {
        match self {
            Registers::R0 => String::from("r0"),
            Registers::R1 => String::from("r1"),
            Registers::RReturn => String::from("rreturn"),
        }
    }
}
