use advent_of_code_2023::day14::day14;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.expect("IO Error"));
    let result = day14(lines);
    println!("{}", result);
}
