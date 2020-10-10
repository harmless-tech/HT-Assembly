use std::mem::size_of;

use crate::components::Type;
use std::borrow::Borrow;

//TODO Switch to ! operator?? (When available)
pub struct TypedData<'a> {
    hta_type: Type,
    data: &'a [u8],
}

pub struct Register<'a> {
    reg1: &'a TypedData<'a>,
    reg2: &'a TypedData<'a>,
    reg_return: &'a TypedData<'a>,
}

pub struct InstructionalMemory {}

/* --- To Bytes --- */
trait Byte {
    fn to_bytes(&self) -> Vec<u8>;
    //fn from_bytes(&self, data: &[u8]) -> Self;
}

impl Byte for i32 {
    fn to_bytes(&self) -> Vec<u8> {
        let bytes: [u8; size_of::<i32>()] = self.to_le_bytes();
        return Vec::from(bytes);
    }
}

/*pub fn to_bytes(data: &i8) -> Vec<u8> {
    let bytes: [u8; size_of::<i32>()] = data.to_le_bytes();
    return Vec::from(bytes);
}

pub fn to_bytes(data: &i32) -> Vec<u8> {
    let bytes: [u8; size_of::<i32>()] = data.to_le_bytes();
    return Vec::from(bytes);
}*/

/* --- From Bytes --- */
