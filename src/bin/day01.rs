use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let mut sum: u32 = 0;
    for line in stdin.lock().lines() {
        sum += get_calibration_value(&line.unwrap()) as u32;
    }
    println!("{}", sum);
}

fn get_calibration_value(line:&str) -> u8 {
    let re_first = Regex::new(r"^[^0-9]*([0-9])").unwrap();
    let re_last = Regex::new(r"([0-9])[^0-9]*$").unwrap();
    let mut number = 0;
    for (_, [first]) in re_first.captures_iter(line).map(|c| c.extract()) {
        number += first.parse::<u8>().unwrap() * 10;
    }
    for (_, [last]) in re_last.captures_iter(line).map(|c| c.extract()) {
        number += last.parse::<u8>().unwrap();
    }
    return number;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(get_calibration_value("1abc2"), 12);
        assert_eq!(get_calibration_value("pqr3stu8vwx"), 38);
        assert_eq!(get_calibration_value("a1b2c3d4e5f"), 15);
        assert_eq!(get_calibration_value("treb7uchet"), 77);
        // total should be 142
    }
}