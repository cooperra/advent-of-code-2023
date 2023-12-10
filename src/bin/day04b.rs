use advent_of_code_2023::day04::day04b;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap());
    let result = day04b(lines);
    println!("{}", result);
}
