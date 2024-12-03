use std::fs;
use std::error::Error;

const FILE_NAME: &str = "data.txt";
fn main() {
    println!("Hello, world!");
    let data_as_string: String = read_to_string(FILE_NAME);
    println!("{data_as_string}");
}

fn read_to_string(file_name: &str) -> String {
    let message: String = fs::read_to_string(file_name).expect("Did not find the file");
    message
}