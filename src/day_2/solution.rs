use crate::util::utils::load_matrix;

/// Input matrix
/// Count the number of safe rows
/// A row is safe if it only ascends or descends
/// AND the change is at least 1 and at most 3
fn verify_safety(matrix: &Vec<Vec<i32>>) -> i32 {
    let mut safe_count = 0;
    for row in matrix {
        safe_count += verify_safety_for_row(row) as i32;
    }
    
    safe_count
}

/// Verify safety of a row.
/// A row is safe if it is only ascending or descending
/// Each change must be at least 1 and at most 3
fn verify_safety_for_row(row: &Vec<i32>) -> bool {
    let mut row_state = RowPattern::NotKnown;
    let mut previous = -1;
    for num in row {
        // update row pattern prior to analyzing
        if previous != -1 && matches!(row_state, RowPattern::NotKnown) {
            if *num > previous {
                row_state = RowPattern::Ascending;
            } else if *num < previous {
                row_state = RowPattern::Descending;
            } else {
                return false;
            }
        }
        
        if previous != -1 {
            match row_state {
                RowPattern::NotKnown => (),
                RowPattern::Ascending => {
                    if *num < previous || !safe_change(previous, *num)  {
                        return false;
                    }
                },
                RowPattern::Descending => {
                    if *num > previous || !safe_change(previous, *num) {
                        return false
                    }
                }
            }
        }
        previous = *num
    }
    true
}

pub fn solve_input() -> i32 {
    let matrix = load_matrix("./src/day_2/input1.txt");
    
    verify_safety(&matrix)
}

fn safe_change(previous: i32, current: i32) -> bool {
    let change = (previous - current).abs();
    change >= 1 && change <= 3
}

enum RowPattern {
    NotKnown,
    Ascending,
    Descending
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn happy_path() {
        let mut matrix = Vec::new();
        for _ in 0..2 {
            matrix.push(vec![1, 2, 3]);
        }
        assert_eq!(verify_safety(&matrix), 2);
    }
    
    #[test]
    fn fail_order_change() {
        let mut matrix = Vec::new();
        matrix.push(vec![1, 2, 3, 3]);
        assert_eq!(verify_safety(&matrix), 0);
    }

    #[test]
    fn fail_big_jump() {
        let mut matrix = Vec::new();
        matrix.push(vec![1, 2, 3, 9]);
        assert_eq!(verify_safety(&matrix), 0);
    }
    
    #[test]
    fn fail_all_same() {
        let mut matrix = Vec::new();
        matrix.push(vec![4, 4, 4]);
        assert_eq!(verify_safety(&matrix), 0);
    }
    
    #[test]
    fn fail_bad_then_good() {
        let mut matrix = Vec::new();
        matrix.push(vec![30, 34, 35]);
        assert_eq!(verify_safety(&matrix), 0);
    }
    
    #[test]
    fn sample_example() {
        let mut matrix = Vec::new();
        matrix.push(vec![7, 6, 4, 2, 1]);
        matrix.push(vec![1, 2, 7, 8, 9]);
        matrix.push(vec![9, 7, 6, 2, 1]);
        matrix.push(vec![1, 3, 2, 4, 5]);
        matrix.push(vec![8, 6, 4, 4, 1]);
        matrix.push(vec![1, 3, 6, 7, 9]);
        assert_eq!(verify_safety(&matrix), 2);
    }
}