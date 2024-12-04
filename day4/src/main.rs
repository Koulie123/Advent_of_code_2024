use std::fs::read_to_string;

fn main() {
    let mut total_count: usize = 0;
    let mut char_vector: Vec<Vec<char>> = Vec::new();
    let file_name: String = String::from("data.txt");
    let vector_string = file_to_readlines(file_name);
    for line in vector_string {
        let temp_line = convert_string_to_char_array(line);
        char_vector.push(temp_line);
    }
    for i in 0..10 {
        println!("{}", char_vector[i][0]);
    }
    for i in 0..4 {
        total_count += search_horizontal(&char_vector[i])
    }
    println!("The total number of xmases in the first 4 lines is {}", total_count);

}

fn file_to_readlines(file_name: String) -> Vec<String> {
    //Convert file to a vector of strings.
    let mut result: Vec<String> = Vec::new();
    for line in read_to_string(file_name).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn convert_string_to_char_array(input_string: String) -> Vec<char> {
    let str_copy = input_string.clone();
    let char_array = str_copy.chars().collect();
    char_array
}

fn search_horizontal(chars: &Vec<char>) -> usize {
    let mut count: usize = 0;
    let length: usize = chars.len();
    let forward_xmas: Vec<char> = vec!['X', 'M', 'A', 'S'];
    let backward_xmas: Vec<char> = vec!['S', 'A', 'M', 'X'];
    for i in 0..(length - 4) {
        if chars[i..i+4] == forward_xmas || chars[i..i+4] == backward_xmas{
            count += 1;
        }
    }
    count   
}
