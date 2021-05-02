use log::error;
use std::process::exit;
use crate::WriteData;
use hta_shared::version;

//TODO Should this use borrowing instead for contents?
pub fn remove_comments(file_name: &str, contents: String) -> Result<String, String> {
    let mut in_quotes = false;
    let mut comment_first_slash = false;

    let mut lines: Vec<String> = contents
        .split("\n")
        .map(|s| String::from(s.trim()))
        .collect();

    // Remove // comments.
    for line in lines.iter_mut() {
        comment_first_slash = false;

        let mut loc = -1_i32;

        for (i, chr) in line.chars().enumerate() {
            if chr == '"' {
                in_quotes = !in_quotes;
                comment_first_slash = false;
            }
            else if chr == '/' {
                if !in_quotes && comment_first_slash {
                    loc = (i as i32) - 1;
                    break;
                }
                else if !in_quotes {
                    comment_first_slash = true;
                }
            }
            else {
                comment_first_slash = false;
            }
        }

        if loc >= 0 {
            line.drain((loc as usize)..line.len());
        }
    }

    let mut in_quotes = false;
    let mut comment_first_slash = false;
    let mut comment_first_star = false;
    let mut in_block_comment = false;

    // Remove /* */ comments.
    let mut index = 0;
    while index < lines.len() {
        let line = match lines.get_mut(index) {
            Some(l) => l,
            None => {
                error!("[FATAL] Compiler somehow left the lines vector range!");
                exit(-1);
            }
        };

        let mut loc1 = -1_i32;
        let mut loc2 = -1_i32;

        comment_first_slash = false;
        comment_first_star = false;

        for (i, chr) in line.chars().enumerate() {
            if chr == '"' {
                in_quotes = !in_quotes;
                comment_first_slash = false;
                comment_first_star = false;
            }
            else if chr == '/' {
                if !in_quotes && comment_first_star && in_block_comment {
                    loc2 = (i as i32) + 1;
                    in_block_comment = false;
                }
                else if !in_quotes && comment_first_star {
                    return Err(format!("Line {}, in file {}, has a */ without a matching /*.", index + 1, file_name));
                }
                else if !in_quotes {
                    comment_first_slash = true;
                }

                comment_first_star = false;
            }
            else if chr == '*' {
                if !in_quotes && comment_first_slash && !in_block_comment {
                    loc1 = (i as i32) - 1;
                    in_block_comment = true;
                }
                else if !in_quotes {
                    comment_first_star = true;
                }

                comment_first_slash = false;
            }
            else {
                comment_first_slash = false;
                comment_first_star = false;
            }
        }

        if loc1 >= 0 && loc2 >= 0 {
            line.drain((loc1 as usize)..(loc2 as usize));
            index -= 1;
        }
        else if loc1 >= 0 {
            line.drain((loc1 as usize)..line.len());
        }
        else if loc2 >= 0 {
            line.drain(0..(loc2 as usize));
        }
        else if in_block_comment {
            line.drain(0..line.len());
        }

        index += 1;
    }

    Ok(lines.join("\n"))
}

pub fn pre_process_entry(write_data: &mut WriteData, file_name: &str, lines: &mut Vec<String>) -> Result<(), String> {
    for (index, line) in lines.iter_mut().enumerate() {
        if line.starts_with("#BUILD") || line.starts_with("#INFO") ||  line.starts_with("#REQUIRE") {
            let spilt: Vec<&str> = line.split_whitespace().collect();
            let spilt: Vec<String>= spilt.iter().map(|s| s.to_string()).collect();

            if spilt.get(0).unwrap().eq("#BUILD") {
                if spilt.len() == 3 {
                    if spilt.get(1).unwrap().eq("binary_name") {
                        write_data.build_data.0 = spilt.get(2).unwrap().clone();
                    }
                    else {
                        return Err(format!("Line {}, in file {}: #BUILD does not have a valid second argument.", index + 1, file_name));
                    }
                }
                else {
                    return Err(format!("Line {}, in file {}: #BUILD should only have 3 arguments.", index + 1, file_name));
                }
            }
            else if spilt.get(0).unwrap().eq("#INFO") {
                if spilt.len() >= 3 {
                    if spilt.get(1).unwrap().eq("name") {
                        write_data.metadata.name = spilt[2..spilt.len()].join(" ");
                    }
                    if spilt.get(1).unwrap().eq("authors") {
                        write_data.metadata.authors = spilt[2..spilt.len()].iter().map(|s| s.clone()).collect();
                    }
                    if spilt.get(1).unwrap().eq("license") {
                        write_data.metadata.license = spilt[2..spilt.len()].join(" ");
                    }
                    else if spilt.len() == 3 {
                        if spilt.get(1).unwrap().eq("version") {
                            write_data.metadata.version = spilt.get(2).unwrap().clone();
                        }
                        else if spilt.get(1).unwrap().eq("website") {
                            write_data.metadata.website = "https://".to_string();
                            write_data.metadata.website.push_str(spilt.get(2).unwrap().clone().as_str());
                        }
                        else if spilt.get(1).unwrap().eq("git") {
                            write_data.metadata.git = "https://".to_string();
                            write_data.metadata.git.push_str(spilt.get(2).unwrap().clone().as_str());
                        }
                    }
                    else {
                        return Err(format!("Line {}, in file {}: #INFO ARG should only have 3 args.", index + 1, file_name));
                    }
                }
                else {
                    return Err(format!("Line {}, in file {}: #INFO should only have 3 arguments. Unless its for name, authors, or license.", index + 1, file_name));
                }
            }
            else if spilt.get(0).unwrap().eq("#REQUIRE") {
                if spilt.len() == 3 {
                    if spilt.get(1).unwrap().eq("hta_version") {
                        let v = match version::parse_version_str(spilt.get(2).unwrap()) {
                            None => return Err(format!("Line {}, in file {}: #REQUIRE hta_version has invalid third arg. Should be in format x.x.x", index + 1, file_name)),
                            Some(v) => v,
                        };

                        if !version::is_version_ge(write_data.compiler_version, v) {
                            return Err(format!("Line {}, in file {}: #REQUIRE hta_version is less then the compiler version, please either increase the version or use an older compiler.", index + 1, file_name))
                        }
                    }
                    else if spilt.get(1).unwrap().eq("native_lib") {
                        return Err(format!("Line {}, in file {}: #REQUIRE native_lib is not implemented.", index + 1, file_name));
                    }
                    else {
                        return Err(format!("Line {}, in file {}: #REQUIRE does not have a valid second argument.", index + 1, file_name));
                    }
                }
                else {
                    return Err(format!("Line {}, in file {}: #REQUIRE should only have 3 arguments.", index + 1, file_name));
                }
            }

            line.drain(0..line.len());
        }
    }

    Ok(())
}

pub fn pre_process_any() {
    //TODO
}
