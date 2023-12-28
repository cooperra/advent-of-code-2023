use advent_of_code_2023::day14::day14b;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.expect("IO Error"));
    let result = day14b(lines, 1000000000);
    println!("{}", result);
}
