pub fn remove_comments(lines: &mut Vec<String>) -> Result<(), String> {
    let mut in_block_comment = false;

    for line in lines.iter_mut() {
        let start_quote = line.find("\"");
        let end_quote = line.rfind("\"");

        // This doesn't matter, /* and // are not valid chars.
        // let start_single_quote = line.find("\'");
        // let end_single_quote = line.rfind("\'");

        let start_block_comment = line.find("/*");
        let end_block_comment = line.find("*/");

        let comment = line.find("//");

        match comment {
            None => {}
            Some(loc) => match (start_quote, end_quote) {
                (Some(q1), Some(q2)) => {
                    if !(loc > q1 && loc < q2) {
                        line.replace_range(loc..line.len(), "");
                    }
                }
                (_, _) => line.replace_range(loc..line.len(), ""),
            },
        }

        //TODO This needs to handle non-matching block comments. (Throw an error)
        if in_block_comment {
            match (start_block_comment, end_block_comment) {
                (Some(loc1), Some(loc2)) => {
                    if loc1 > loc2 { // */ /*

                    }
                }
                (_, Some(loc2)) => match (start_quote, end_quote) {
                    (Some(q1), Some(q2)) => {
                        if !(loc2 > q1 && loc2 < q2) {
                            line.replace_range(0..(loc2 + 3), "");
                            in_block_comment = false;
                        }
                    }
                    (_, _) => {
                        line.replace_range(0..(loc2 + 3), "");
                        in_block_comment = false;
                    }
                },
                (_, _) => {}
            }
        }
        else {
            match (start_block_comment, end_block_comment) {
                (Some(loc1), Some(loc2)) => match (start_quote, end_quote) {
                    (Some(q1), Some(q2)) => {
                        if !(loc1 > q1 && loc1 < q2) && !(loc1 > q1 && loc1 < q2) {
                            line.replace_range(loc1..(loc2 + 3), "");
                        }
                        else if !(loc1 > q1 && loc1 < q2) {
                            line.replace_range(loc1..line.len(), "");
                        }
                    }
                    (_, _) => line.replace_range(loc1..(loc2 + 3), ""),
                },
                (Some(loc1), _) => match (start_quote, end_quote) {
                    (Some(q1), Some(q2)) => {
                        if !(loc1 > q1 && loc1 < q2) {
                            line.replace_range(loc1..line.len(), "");
                            in_block_comment = true;
                        }
                    }
                    (_, _) => {
                        line.replace_range(loc1..line.len(), "");
                        in_block_comment = true;
                    }
                },
                (_, _) => {}
            }
        }
    }

    Ok(())
}
