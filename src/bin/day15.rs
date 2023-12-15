use advent_of_code_2023::day15::*;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let result = day15(lines.next().unwrap());
    println!("{}", result);
}

pub fn day15(line: impl AsRef<str>) -> Num {
    let mut sum = 0;
    for chunk in line.as_ref().split(",") {
        let chunk_hash = hash(&chunk);
        println!("'{}' -> {}", chunk, chunk_hash);
        sum += chunk_hash as Num;
    }
    sum
}
