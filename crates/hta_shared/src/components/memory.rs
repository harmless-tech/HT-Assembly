use std::{any::Any, borrow::Borrow, mem::size_of};

use crate::components::Types;

//TODO Switch to ! operator?? (When available)
// TypedData is now a tuple (Types, Box<dyn Any>).
/*pub struct TypedData {
    hta_type: Type,
    data: dyn Any
}*/

//TODO Move this to runtime!
/*pub struct RegisterStorage<'a> {
    reg1: &'a (Types, Box<dyn Any>),
    reg2: &'a (Types, Box<dyn Any>),
    reg_return: &'a (Types, Box<dyn Any>)
}*/

//TODO What was this for??
pub struct InstructionalMemory {}
