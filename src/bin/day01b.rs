use std::io::{self, BufRead};
use regex::{Regex, Captures, Match};

fn main() {
    let stdin = io::stdin();
    let mut sum: u32 = 0;
    for line in stdin.lock().lines() {
        sum += get_calibration_value(&line.unwrap()) as u32;
    }
    println!("{}", sum);
}

fn get_calibration_value(line:&str) -> u8 {
    let re = Regex::new(r"(?:(?<digit>[0-9])|(?<overlap>twoneight|twone|oneight|nineight|eightwo)|(?<word>zero|one|two|three|four|five|six|seven|eight|nine))").unwrap();
    let mut first_num = None;
    let mut last_num = 0;
    for capture in re.captures_iter(line) {
        if first_num.is_none() {
            first_num = Some(match capture.name("overlap").map(|m: Match| m.as_str()) {
               None => match_to_num(&capture),
               Some("twoneight") => 2,
               Some("twone") => 2,
               Some("oneight") => 1,
               Some("nineight") => 9,
	       Some("eightwo") => 8,
               _ => panic!(),
            });
        }
        last_num = match capture.name("overlap").map(|m: Match| m.as_str()) {
           None => match_to_num(&capture),
           Some("twoneight") => 8,
           Some("twone") => 1,
           Some("oneight") => 8,
           Some("nineight") => 8,
	   Some("eightwo") => 2,
           _ => panic!(),
        };
    }
    let number = first_num.unwrap_or(0) * 10 + last_num;
    return number;
}

fn match_to_num(c: &Captures) -> u8 {
    if let Some(d) = c.name("digit") {
        return d.as_str().parse::<u8>().unwrap();
    }
    match c.name("word").unwrap().as_str() {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        &_ => panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(get_calibration_value("two1nine"), 29);
        assert_eq!(get_calibration_value("eightwothree"), 83);
        assert_eq!(get_calibration_value("abcone2threexyz"), 13);
        assert_eq!(get_calibration_value("xtwone3four"), 24);
        assert_eq!(get_calibration_value("4nineeightseven2"), 42);
        assert_eq!(get_calibration_value("zoneight234"), 14);
        assert_eq!(get_calibration_value("7pqrstsixteen"), 76);
        // total should be 281
    }

    #[test]
    fn tricky_one() {
        assert_eq!(get_calibration_value("8threesevenfourgbgteight5twonenjr"), 81);
        assert_eq!(get_calibration_value("bnjqlftwobvsvjqptdp1two94twonej"), 21);
        assert_eq!(get_calibration_value("7nmrndvq7jnxnlsseven9twonelxb"), 71);
        assert_eq!(get_calibration_value("nrtwonetlmkldqrcjqrdn6gptzdclninethreenine"), 29);
        assert_eq!(get_calibration_value("rdktwone9fourkklk9rsseven"), 27);
        assert_eq!(get_calibration_value("one77twoeighteightfive6twonek"), 11);
        assert_eq!(get_calibration_value("kpqsxmvhp4twohnlsone3eighttwones"), 41);
        assert_eq!(get_calibration_value("zktwonemhqnxssxftwotsd1nhfmrxpffoureight7"), 27);
        assert_eq!(get_calibration_value("mgtwoneonecthreefoureight37eightjqlxf"), 28);
        assert_eq!(get_calibration_value("ztwoneeightfourzzsck7seventwo"), 22);
        assert_eq!(get_calibration_value("rnprnnpbjq7fivetwoneqsh"), 71);
        assert_eq!(get_calibration_value("1twonexlr"), 11);
        assert_eq!(get_calibration_value("fiveklbblk4eighttwonefdf"), 51);
        assert_eq!(get_calibration_value("45twoneqs"), 41);
        assert_eq!(get_calibration_value("mftwone3eighthhcsgfvrrj"), 28);
        assert_eq!(get_calibration_value("hftwoneninesixxxmdtcfd8lbvqdjg"), 28);
        assert_eq!(get_calibration_value("eightpmbdvzmdmpfivebphsv2pzxtcsrvtgnqnhvsbdf9twonehc"), 81);
        assert_eq!(get_calibration_value("stwone15"), 25);
        assert_eq!(get_calibration_value("rpxtwone83"), 23);
        assert_eq!(get_calibration_value("djctwonefourlxshzxzmff313onesixkzxxhrrfour"), 24);
    }

    #[test]
    fn more_trickery() {
       assert_eq!(get_calibration_value("oneight"), 18);
       assert_eq!(get_calibration_value("nineight"), 98);
       assert_eq!(get_calibration_value("twoneight"), 28);
       assert_eq!(get_calibration_value("eightwo"), 82);
       // They weren't clever enough to include "twoneight" or "oneightwo" or "nineightwo" or worse.
       // Also no "eighthree", "fiveight", "sevenine"
    }
}