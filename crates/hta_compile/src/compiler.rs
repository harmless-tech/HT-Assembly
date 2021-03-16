use log::error;
use std::process::exit;

pub fn remove_comments(contents: String) -> Result<String, String> {
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
                    return Err(format!("Line {} has a */ without a matching /*.", index + 1));
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
