use std::{fs::read_to_string, usize};
const FILE_NAME: &str = "data.txt";
fn main() {
    let mut main_dictionary: Vec<reference_number> = Vec::new();
    let mut string_data = read_lines(FILE_NAME);
    for i in 0..1176 {
        let temp_num: Vec<usize> = convert_line_to_nums(&string_data[i]);
        let is_in_dict: bool = check_if_in_dictionary(&main_dictionary, temp_num[0]);
    }
}

fn check_if_in_dictionary(dictionary: &Vec<reference_number>, number: usize) -> bool {
    for i in dictionary {
        if i.main_number == number {
            return true;
        }
    }
    false
}
struct reference_number {
    main_number: usize,
    numbers_after: Vec<usize>
}
impl reference_number {
    fn add_to_dictionary(&mut self, item_to_add: usize) {
        self.numbers_after.push(item_to_add);
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn convert_line_to_nums(input: &String) -> Vec<usize> {
    let line: String = input.clone();
    line.trim()
        .split('|')
        .map(|num| num.parse().unwrap())
        .collect()
    
}