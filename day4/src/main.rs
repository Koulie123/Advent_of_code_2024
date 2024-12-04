use std::fs::read_to_string;
const FORWARD_XMAS: [char; 4]= ['X', 'M', 'A', 'S'];
const BACKWARD_XMAS: [char; 4] = ['S', 'A', 'M', 'X'];
fn main() {

    let mut total_count: usize = 0;
    let mut char_vector: Vec<Vec<char>> = Vec::new();
    let file_name: String = String::from("data.txt");
    let vector_string = file_to_readlines(file_name);
    for line in vector_string {
        let temp_line = convert_string_to_char_array(line);
        char_vector.push(temp_line);
    }
    for i in 0..char_vector.len() {
        total_count += search_horizontal(&char_vector[i])
    }
    total_count += search_vertical(&char_vector);
    total_count += search_diagonal_down(&char_vector);
    total_count += search_diagonal_up(&char_vector);
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
    for i in 0..(length - 3) {
        if chars[i..i+4] == *FORWARD_XMAS.as_slice() || chars[i..i+4] == *BACKWARD_XMAS.as_slice(){
            count += 1;
        }
    }
    count   
}

fn search_vertical(chars: &Vec<Vec<char>>) -> usize {
    let height: usize = chars[0].len();
    let length: usize = chars.len();
    let mut count: usize = 0;
    for j in 0..height {
        for i in 0..length-3 {
            let mut slice: Vec<char> = Vec::new();
            for k in 0..4 {
                slice.push(chars[i+k][j])
            }
            if slice == FORWARD_XMAS || slice == BACKWARD_XMAS {
                count += 1;
            }
        }
    }
    count
}
fn search_diagonal_down(chars: &Vec<Vec<char>>) -> usize{
    let height: usize = chars[0].len();
    let length: usize = chars.len();
    let mut count: usize = 0;
    for i in 0..length-3 {
        for j in 0..height-3 {
            let mut slice: Vec<char> = Vec::new();
            for k in 0..4 {
                slice.push(chars[i+k][j+k]);
            }
            if slice == FORWARD_XMAS || slice == BACKWARD_XMAS {
                count += 1;
            }
        }
    }
    count
}

fn search_diagonal_up(chars: &Vec<Vec<char>>) -> usize {
    let height: usize = chars[0].len();
    let length: usize = chars.len();
    let mut count: usize = 0;
    for i in 0..length-3 {
        for j in 3..height {
            let mut slice: Vec<char> = Vec::new();
            for k in 0..4 {
                slice.push(chars[i+k][j-k]);
            }
            if slice == FORWARD_XMAS || slice == BACKWARD_XMAS {
                count += 1;
            }
        }
    }
    count
}