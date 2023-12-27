use advent_of_code_2023::day13::day13;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.expect("IO Error"));
    let result = day13(lines);
    println!("{}", result);
}
