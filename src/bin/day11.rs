use std::io::{self, BufRead};

use advent_of_code_2023::day11::day11;

fn main() {
    let stdin = io::stdin().lock();
    let lines = stdin.lines().map(|l| l.expect("IO Error"));
    let result = day11(lines, 2);
    println!("{}", result);
}
