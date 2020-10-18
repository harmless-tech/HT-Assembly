mod arg_crafter;
mod test;

use std::{any::Any, collections::HashMap};

use hta_shared::{
    components::{Instructions, Types},
    hta_database::{HTADatabase, HTAFrame}
};
use log::{debug, error};

//use crate::arg_crafter as ac;

//TODO Needs a return.
//TODO Multi-threading compiling. Multiple files compiling.
pub fn compile<'a>(content: &str) -> HTADatabase {
    let str: String = String::from(content);
    let mut lines: Vec<String> = str.split("\n").map(|s| String::from(s.trim())).collect();

    remove_comments_and_lines(&mut lines);
    remove_semi_colon(&mut lines);

    debug!("START Imported file:");
    lines.iter().for_each(|s| debug!("{}", s));
    debug!("END Imported file:");

    //TODO When multiple files are compile do linking check!
    let mut map: HashMap<String, HTAFrame> = HashMap::new();

    //TODO Allow for renaming of init files. Allow for multiple files.
    map.insert("main".to_string(), compile_process(&lines));

    return HTADatabase {
        entry_frame: String::from("main"),
        frames: map
    };
}

//TODO This function will do linking checks for the multiple files.
//TODO Mark2
fn linker() {}

fn remove_comments_and_lines(lines: &mut Vec<String>) {
    let mut in_comment: bool = false;
    for line in lines.iter_mut() {
        // Remove // comments.
        let index: Option<usize> = line.find("//");
        if index.is_some() {
            line.replace_range(index.unwrap()..line.len(), "");
        }

        // Remove ** comments.
        let start_index: Option<usize> = line.find("/*");
        let end_index: Option<usize> = line.find("*/");

        //TODO FIXME Comments where there is a */ and then a /* on the same line are broken.
        if start_index.is_some() {
            in_comment = true;
        }

        if in_comment {
            match (start_index.is_some(), end_index.is_some()) {
                (true, false) => line.replace_range(start_index.unwrap()..line.len(), ""),
                (true, true) => line.replace_range(start_index.unwrap()..end_index.unwrap(), ""),
                (false, true) => line.replace_range(0..end_index.unwrap() + 2, ""),
                _ => line.replace_range(0..line.len(), "")
            }
        }

        //TODO Better way than an if statement.
        if in_comment && end_index.is_some() {
            in_comment = false;
        }
    }

    lines.retain(|s| !s.is_empty());
}

fn remove_semi_colon(lines: &mut Vec<String>) {
    lines.iter_mut().for_each(|s| {
        if s.ends_with(";") {
            s.remove(s.len() - 1);
        }
    });
}

//TODO Do safety checks!
fn compile_process(lines: &Vec<String>) -> HTAFrame {
    let mut instructions: Vec<Instructions> = Vec::new();
    let mut tags: HashMap<String, u32> = HashMap::new();
    let vars: HashMap<String, (Types, Box<dyn Any>)> = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        if line.ends_with(":") {
            let mut tag: String = line.clone();
            tag.remove(tag.len() - 1);

            tags.insert(tag, i as u32);
        }

        let instr: Instructions = compile_line(line);
        instructions.push(instr);
    }

    return HTAFrame {
        instructions,
        tags,
        vars
    };
}

//TODO Do safety checks!
fn compile_line(line: &str) -> Instructions {
    //TODO This doesn't help if the last arg is a string arg.
    let args: Vec<&str> = line.split_whitespace().collect();

    if args.get(0).unwrap().ends_with(":") {
        return Instructions::Blank;
    }

    //TODO More safety when collecting args.
    //TODO Proper collection of args.
    //TODO Keep track of alloc vars and cast setvar to their type.
    return match args.get(0).unwrap().to_lowercase().as_str() {
        "alloc" => Instructions::Alloc {
            name: arg_crafter::arg_name(&args, 1),
            hta_type: arg_crafter::arg_hta_type(&args, 2),
            default: None //TODO Later. Allow for default vals.
        },
        "setvar" => Instructions::SetVar {
            name: arg_crafter::arg_name(&args, 1),
            //TODO This data is not correct. Data needs to be cast, before being stored.
            // Need to find a way to make strings to work.
            data: Box::from(/*String::from(*args.get(2).unwrap())*/ 12)
        },
        "regvar" => Instructions::RegVar {
            name: arg_crafter::arg_name(&args, 1)
        },
        "setreg" => Instructions::SetReg {
            register: arg_crafter::arg_register(&args, 1),
            hta_type: arg_crafter::arg_hta_type(&args, 2),
            default: None //TODO Later.
        },
        "varreg" => Instructions::VarReg {
            name: arg_crafter::arg_name(&args, 1),
            register: arg_crafter::arg_register(&args, 2)
        },
        "cpyreg" => Instructions::CpyReg {
            register: arg_crafter::arg_register(&args, 1)
        },
        "op" => Instructions::Op {
            operation: arg_crafter::arg_operation(&args, 1)
        },
        //"pushjmp" => {}, //TODO Impl.
        "popjmp" => Instructions::PopJmp,
        //"loop" => {},   //TODO Impl.
        //"cast" => {},   //TODO Impl.
        //"native" => {}, //TODO Impl.
        //"return" => {}, //TODO Impl.
        "exit" => Instructions::Exit {
            code: -1 //TODO Different codes.
        },
        _ => {
            //TODO Not right. No return, end compiler with error.
            error!("Unknown instruction: {}", line);
            return Instructions::Blank;
        }
    };
}
