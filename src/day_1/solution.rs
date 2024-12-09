use std::collections::{HashMap, HashSet};
use crate::util::utils::load_input;

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

/// Take in 2 lists.
/// Find the number of times a number in input 1 occurs in input 2
/// Multiply the number by occurrences
/// Return the sum of all multiplied numbers
fn calculate_similarity_score(input_1: Vec<i32>, input_2: Vec<i32>) -> i32 {
    let occurrences = find_occurrences(&input_1, input_2);
    let mut similarity_score = 0;
    for num in input_1 {
        if let Some(value) = occurrences.get(&num) {
            similarity_score += num * value;
        }
    }

    similarity_score
}

fn find_occurrences(input_1: &Vec<i32>, input_2: Vec<i32>) -> HashMap<i32, i32> {
    let unique: HashSet<i32> = HashSet::from_iter(input_1.iter().cloned());
    let mut result: HashMap<i32, i32> = HashMap::new();
    for num in input_2.iter() {
        if unique.contains(num) {
            if result.contains_key(num) {
                result.insert(*num, result.get(num).unwrap() + 1);
            } else {
                result.insert(*num, 1);
            }
        }
    }

    result
}

/// Read input txt and solve
pub fn solve_input() -> i32 {
    let contents = load_input("./src/day_1/input1.txt");

    sum_distances(contents.0, contents.1)
}

pub fn solve_input_part_2() -> i32 {
    let contents = load_input("./src/day_1/input1.txt");
    
    calculate_similarity_score(contents.0, contents.1)
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

    #[test]
    fn happy_path_similarity_score() {
        let input_1 = vec![1, 2, 2, 3];
        let input_2 = vec![1, 2, 2, 3];
        assert_eq!(calculate_similarity_score(input_1, input_2), 12);
    }

    #[test]
    fn sample_example_similarity_score() {
        let input_1 = vec![3, 4, 2, 1, 3, 3];
        let input_2 = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(calculate_similarity_score(input_1, input_2), 31);
    }
}