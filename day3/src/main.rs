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

fn return_locations_of_muls (string_array: &String) -> Vec<usize> {
    let char_vector: Vec<char> = string_array.chars().collect();
    let mut vector_to_return: Vec<usize> = Vec::new();
    // let mut i: usize = 0;
    for i in 0..(char_vector.len()-2) {
        if check_for_mul(&char_vector[i..i+3]){
            vector_to_return.push(i);
        }

    }
    vector_to_return
}

fn check_for_mul(slice: &[char]) -> bool {
    let mut flag: bool = false;
    if slice[0] == 'm' && slice[1] == 'u' && slice[2] == 'l'{
        flag = true;
    }
    flag
}