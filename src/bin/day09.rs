use std::io::{self, BufRead};

use regex_macro::regex;

type Num = i32;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.expect("IO Error"));
    let result = day09(lines);
    println!("{}, {}", result.0, result.1);
}

fn day09(lines: impl Iterator<Item = impl AsRef<str>>) -> (Num, Num) {
    let mut sum = (0, 0);
    for line in lines {
        let (prev, next) = process_line(line.as_ref());
        sum.0 += prev;
        sum.1 += next;
    }
    sum
}

fn process_line(line: &str) -> (Num, Num) {
    let nums = parse_line(line);
    predict_prev_next(&nums)
}

fn parse_line(line: &str) -> Vec<Num> {
    let re = regex!(r"-?[0-9]+");
    re.find_iter(line)
        .map(|m| m.as_str().parse::<Num>().unwrap())
        .collect()
}

fn predict_prev_next(nums: &Vec<Num>) -> (Num, Num) {
    assert!(nums.len() > 0);
    if nums.iter().all(|n| *n == 0) {
        return (0, 0);
    }
    let next_row = nums.windows(2).map(|slice| slice[1] - slice[0]).collect();
    let (next_row_prev, next_row_next) = predict_prev_next(&next_row);
    return (
        nums.first().unwrap() - next_row_prev,
        nums.last().unwrap() + next_row_next,
    );
}
