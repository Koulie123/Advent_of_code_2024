use std::fs::read_to_string;

fn main() {
    let file_name: &str = "./data.txt";
    let file_lines = read_lines(file_name);
    let mut running_difference = 0;
    let length_of_list = file_lines.len();
    let mut iterator: usize = 0;
    let mut sorted_list_1: Vec<isize> = Vec::new();
    let mut sorted_list_2: Vec<isize> = Vec::new();
    while iterator < length_of_list {
        let numbers: Vec<&str> = file_lines[iterator]
            .trim()
            .split_whitespace()
            .collect();
        if numbers.len() >= 2 {
            let num1: isize = numbers[0].trim().parse().expect("Failed to read a number");
            let num2: isize = numbers[1].trim().parse().expect("Failed to read a number");
            sorted_list_1.push(num1);
            sorted_list_2.push(num2);
        }
        iterator = iterator + 1;
    }
    sorted_list_1.sort();
    sorted_list_2.sort();
    let running_difference = compare_sorted_list(sorted_list_1, sorted_list_2);
    println!("The total difference is {}", running_difference.to_string());
}
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
fn compare_sorted_list(list1: Vec<isize>, list2: Vec<isize>) -> isize {
    let length: usize = list1.len();
    let mut i: usize = 0;
    let mut running_total: isize = 0;
    while i < length {
        running_total = running_total + (list1[i] - list2[i]).abs();
        i = i + 1;
    }
    running_total
}