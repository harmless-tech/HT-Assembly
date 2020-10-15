pub use self::{
    instructions::{Instruction, Instructions},
    memory::Register,
    types::{Registers, Type, Operations}
};

mod instructions;
mod memory;
mod types;
