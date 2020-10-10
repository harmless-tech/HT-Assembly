use std::{
    fs::{remove_file, File},
    io::{prelude::*, BufReader, Result}
};

use log::error;

pub fn import_file(location: &str) -> String {
    let file: Result<File> = File::open(location);
    let mut contents: String = String::new();

    match file {
        Ok(f) => {
            let mut reader: BufReader<File> = BufReader::new(f);
            reader
                .read_to_string(&mut contents)
                .expect(["Could not import ", location, "!"].join("").as_str());
        },
        Err(e) => error!("Failed to find file at {}!\n {}", location, e)
    }

    return contents;
}

pub fn delete_file(location: &str) -> std::io::Result<()> {
    remove_file(location)?;
    return Ok(());
}
