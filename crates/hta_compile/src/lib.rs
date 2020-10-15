mod test;

use log::debug;

use hta_shared::{
    components,
    components::{Instruction, Type}
};

//TODO Needs a return.
//TODO Multi-threading compiling.
pub fn compile<'a>(content: &str) {
    let str: String = String::from(content);
    let mut lines: Vec<String> = str.split("\n").map(|s| String::from(s.trim())).collect();

    remove_comments_and_lines(&mut lines);
    remove_semi_colon(&mut lines);

    debug!("START Imported file:");
    lines.iter().for_each(|s| debug!("{}", s));
    debug!("END Imported file:");
}

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

fn compile_process() {}

fn compile_line() {}
