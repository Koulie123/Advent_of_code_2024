use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

fn file_to_readlines(file_name: String) -> Vec<String> {
    //Convert file to a vector of strings.
    let mut result: Vec<String> = Vec::new();
    for line in read_to_string(file_name).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn convert_string_to_char_array(input_string: &String) -> Vec<char> {
    let str_copy = input_string.clone();
    let char_array = str_copy.chars().collect();
    char_array
}