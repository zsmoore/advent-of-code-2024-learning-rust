use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Take in 2 lists.
/// Sort them
/// Take the distance between numbers at each index
/// Output sum of all distances
fn sum_distances(input_1: Vec<i32>, input_2: Vec<i32>) -> i32 {
    let mut sorted_1 = input_1.to_vec();
    sorted_1.sort();

    let mut sorted_2 = input_2.to_vec();
    sorted_2.sort();

    let mut summed_distances = 0;
    for i in 0..sorted_1.len() {
        summed_distances += (sorted_1[i] - sorted_2[i]).abs();
    }
    summed_distances
}

/// Read input txt and solve
pub fn solve_input()-> i32 {
    let contents = load_lists("./src/day_1/input.txt");

    sum_distances(contents.0, contents.1)
}

fn load_lists(file_path: &str) -> (Vec<i32>, Vec<i32>) {
    let mut input_1 = Vec::new();
    let mut input_2 = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {
            let mut split = line.split_whitespace();
            input_1.push(split.next().unwrap().parse::<i32>().unwrap());
            input_2.push(split.next().unwrap().parse::<i32>().unwrap());
        }
    }

    (input_1, input_2)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        let input_1 = vec![1, 2, 3];
        let input_2 = vec![1, 2, 3];
        assert_eq!(sum_distances(input_1, input_2), 0);
    }

    #[test]
    fn sample_example() {
        let input_1 = vec![3, 4, 2, 1, 3, 3];
        let input_2 = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(sum_distances(input_1, input_2), 11);
    }

    #[test]
    fn sample_order_reversed() {
        let input_1 = vec![4, 3, 5, 3, 9, 3];
        let input_2 = vec![3, 4, 2, 1, 3, 3];
        assert_eq!(sum_distances(input_1, input_2), 11);
    }
}