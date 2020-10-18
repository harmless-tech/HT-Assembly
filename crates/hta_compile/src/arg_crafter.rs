use std::any::Any;

use hta_shared::components::{Operations, Registers, Types};

//TODO Throw error if name is invalid.
pub fn arg_name(args: &Vec<&str>, index: usize) -> String {
    return String::from(*args.get(index).unwrap());
}

//TODO Throw error if type is invalid.
pub fn arg_hta_type(args: &Vec<&str>, index: usize) -> Types {
    return Types::get(*args.get(index).unwrap()).unwrap();
}

//TODO Throw error if default value is invalid.
/*pub fn arg_default(args: &Vec<&str>, index: usize) -> Option<Box<dyn Any>> {
}*/

//TODO Throw error if data value is invalid.
/*pub fn arg_data(args: &Vec<&str>, index: usize) -> Box<dyn Any> {
}*/

//TODO Throw error if register is invalid.
pub fn arg_register(args: &Vec<&str>, index: usize) -> Registers {
    return Registers::get(*args.get(index).unwrap()).unwrap();
}

//TODO Throw error if operation is invalid.
pub fn arg_operation(args: &Vec<&str>, index: usize) -> Operations {
    return Operations::get(*args.get(index).unwrap()).unwrap();
}
