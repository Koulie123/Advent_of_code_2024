use std::fs::read_to_string;

const FILE_NAME: &str = "data.txt";


fn main() {
    println!("Hello, world!");

    let string_vector: Vec<String> = file_to_string_vec(FILE_NAME);
    println!("This is the first line of the string vector {}", string_vector[0]);
}
fn file_to_string_vec(file_name: &str) -> Vec<String> {
    let mut string_vector: Vec<String> = Vec::new();
    for line in read_to_string(file_name).unwrap().lines() {
        string_vector.push(line.to_string())
    }


    string_vector
}

fn string_vec_to_int_vec(input_string: &String) -> Vec<usize> {
    let vec_strings: Vec<&str> = input_string.split_whitespace().collect();
    let mut vec_usize: Vec<usize> = Vec::new();
    for item in vec_strings {
        let parsed = item.trim().parse::<usize>().expect("Not able to parse");
        vec_usize.push(parsed);
    }
    vec_usize
}
fn check_safety_of_vec(input_vec: Vec<usize>) -> bool {
    let length: usize = input_vec.len() - 1;
    let i: usize = 0;
    let flag = true;
    while i < length {
        let abs_diff: usize = (input_vec[i].into(isize) - input_vec[i + 1].into(isize)).abs
        if input_vec[i]
    }
    flag
}