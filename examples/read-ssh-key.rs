use std::{env, fs, io};

fn main() -> Result<(), io::Error> {
    // This example tries to read first file from .ssh directory
    let ssh_path = env::home_dir().unwrap().join(".ssh");

    let first_file = fs::read_dir(&ssh_path)?
        .map(|e| e.unwrap())
        .find(|e| e.metadata().unwrap().is_file())
        .expect("There should be at least one file in .ssh");
    fs::read_to_string(first_file.path()).map(|_| ())
}
