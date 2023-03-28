use std::{
    fs::File,
    io::{Error, Read},
    path::PathBuf,
};

pub fn get_file_string(path: &PathBuf) -> Result<String, Error> {
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}
