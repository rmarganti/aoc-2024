use std::{env, fs};

pub fn read_file_from_args() -> String {
    let filename = env::args()
        .nth(1)
        .expect("Please provide a filename as an argument");

    fs::read_to_string(filename).expect("Failed to read file")
}
