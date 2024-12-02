use std::fs::read_to_string;

const FILE_NAME: &str = "data.txt";


fn main() {
    println!("Hello, world!");
    let string_vector: Vec<String> = file_to_string_vec(FILE_NAME);
    println!("Hello, world!2");
    println!("This is the first line of the string vector {}", string_vector[0]);
    let final_count: usize = check_all_lines(&string_vector);
    println!("The final count is {}", final_count);

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

fn check_safety_of_vec(input_vec: &Vec<usize>) -> bool {
    let length: usize = input_vec.len() - 1;
    let mut i: usize = 0;
    let mut flag = true;
    while i < length {
        let abs_diff: usize = (input_vec[i] as isize - input_vec[i + 1] as isize).abs() as usize;
        if abs_diff < 1 || abs_diff > 3 {
            flag = false;
        }
        i += 1;
    }

    flag && is_increasing_or_decreasing(&input_vec)
}

fn is_increasing_or_decreasing(input_vec: &Vec<usize>) -> bool {
    let is_increasing: bool;
    let mut is_consistent: bool = true;
    let length: usize = input_vec.len() - 1;
    let mut i: usize = 0;
    if input_vec[1] > input_vec[0] {
        is_increasing = true;
    } else {
        is_increasing = false;
    }
    while i < length {
        if is_increasing && input_vec[i] >= input_vec[i + 1]{
            is_consistent = false;
        }
        if !is_increasing && input_vec[i] <= input_vec[i + 1]{
            is_consistent = false;
        }
        i += 1;
    }
    is_consistent

}
fn convert_all_lines_to_ints(string_vector: &Vec<String>) -> Vec<Vec<usize>> {
    let mut return_vector: Vec<Vec<usize>> = Vec::new();
    for line in string_vector {
        return_vector.push(string_vec_to_int_vec(&line))
    }
    return_vector
}

fn check_all_lines(input_lines: &Vec<String>) -> usize {
    let mut count: usize = 0;
    let int_vector: Vec<Vec<usize>> = convert_all_lines_to_ints(&input_lines);
    for line in int_vector {
        println!("Checking a line!");
        if check_safety_of_vec(&line) == true {
            count += 1;
            println!("Found a safe line, current count = {}", count);
            for i in &line {
                print!(" {}", i);
            }
        }
    }
    count
}