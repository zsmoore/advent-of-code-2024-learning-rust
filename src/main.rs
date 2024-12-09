mod day_1;
mod util;
mod day_2;

fn main() {
    // day 1
    solve_day_1();
    // day 2
    solve_day_2();
}

fn solve_day_1() {
    println!("Day 1 solution: {}", day_1::solution::solve_input());
    println!("Day 1 solution part 2: {}", day_1::solution::solve_input_part_2());
}

fn solve_day_2() {
    println!("Day 2 solution: {}", day_2::solution::solve_input());
}
