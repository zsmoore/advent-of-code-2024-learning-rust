use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Load 2 lists of numbers from file
pub fn load_input(file_path: &str) -> (Vec<i32>, Vec<i32>) {
    let mut input_1 = Vec::new();
    let mut input_2 = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {
            let mut split = line.split_whitespace();
            input_1.push(split.next().unwrap().parse().unwrap());
            input_2.push(split.next().unwrap().parse().unwrap());
        }
    }

    (input_1, input_2)
}

pub fn load_matrix(file_path: &str) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {
            let split: Vec<i32> = line.split_whitespace().map(|str| str.parse::<i32>().unwrap()).collect();
            result.push(split);
        }
    }
    
    result
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}