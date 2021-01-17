use std::any::Any;

use log::{debug, error};

use hta_shared::components::{Operations, Registers, Types};

//TODO String storage: Some(Box::<Vec<char>>::new("Gamer!".chars().collect()))

//TODO Throw error if name is invalid.
pub fn arg_name(args: &Vec<&str>, index: usize) -> String {
    return String::from(*args.get(index).unwrap());
}

//TODO Throw error if type is invalid.
pub fn arg_hta_type(args: &Vec<&str>, index: usize) -> Types {
    return Types::get(*args.get(index).unwrap()).unwrap();
}

//TODO Throw error if default value is invalid.
pub fn arg_default(line: &str, args: &Vec<&str>, type_index: usize) -> Option<Box<dyn Any>> {
    if args.len() >= type_index + 1 {
        let hta_type: Types = arg_hta_type(args, type_index);
        let index: usize = type_index + 1;

        match hta_type {
            Types::Char => {
                if line.ends_with("'") {
                    let top_quote: usize = line.find("'").unwrap();
                    let ch: &str = &line[(top_quote + 1)..(line.len() - 1)];

                    //debug!("CHAR: {}", &line[(top_quote + 1)..(line.len() - 1)]);
                    //TODO Does not work with \n and other chars like it.

                    return Some(Box::<char>::new(ch.parse::<char>().unwrap()));
                }

                error!("Char has invalid format!");
                std::process::exit(-2);
            },
            Types::String => return None, //TODO Allow for strings.
            Types::Int8 => {
                return Some(Box::<i8>::new(
                    args.get(index).unwrap().parse::<i8>().unwrap()
                ));
            },
            Types::Int16 => {
                return Some(Box::<i16>::new(
                    args.get(index).unwrap().parse::<i16>().unwrap()
                ));
            },
            Types::Int32 => {
                return Some(Box::<i32>::new(
                    args.get(index).unwrap().parse::<i32>().unwrap()
                ));
            },
            Types::Int64 => {
                return Some(Box::<i64>::new(
                    args.get(index).unwrap().parse::<i64>().unwrap()
                ));
            },
            Types::Int128 => {
                return Some(Box::<i128>::new(
                    args.get(index).unwrap().parse::<i128>().unwrap()
                ));
            },
            Types::UInt8 => {
                return Some(Box::<u8>::new(
                    args.get(index).unwrap().parse::<u8>().unwrap()
                ));
            },
            Types::UInt16 => {
                return Some(Box::<u16>::new(
                    args.get(index).unwrap().parse::<u16>().unwrap()
                ));
            },
            Types::UInt32 => {
                return Some(Box::<u32>::new(
                    args.get(index).unwrap().parse::<u32>().unwrap()
                ));
            },
            Types::UInt64 => {
                return Some(Box::<u64>::new(
                    args.get(index).unwrap().parse::<u64>().unwrap()
                ));
            },
            Types::UInt128 => {
                return Some(Box::<u128>::new(
                    args.get(index).unwrap().parse::<u128>().unwrap()
                ));
            },
            Types::Float32 => {
                return Some(Box::<f32>::new(
                    args.get(index).unwrap().parse::<f32>().unwrap()
                ));
            },
            Types::Float64 => {
                return Some(Box::<f64>::new(
                    args.get(index).unwrap().parse::<f64>().unwrap()
                ));
            },
            Types::Boolean => {
                return Some(Box::<bool>::new(
                    args.get(index).unwrap().parse::<bool>().unwrap()
                ));
            },
            Types::Object => {
                error!("Object type is not support in mark1!");
                std::process::exit(-2);
            },
            Types::Void => {
                error!("Void type is not support in mark1!");
                std::process::exit(-2);
            }
        }
    }

    return None;
}

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
