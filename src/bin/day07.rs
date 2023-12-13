use advent_of_code_2023::day07::*;
use std::io::{self, BufRead};

type Num = u32;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap());
    let result = day07(lines);
    println!("{}", result);
}

pub fn day07(lines: impl Iterator<Item = impl AsRef<str>>) -> Num {
    let mut hands: Vec<(Hand, Num)> = lines
        .map(|line| {
            let mut split_line = line.as_ref().split_whitespace();
            let hand =
                Hand::from_str(split_line.next().expect("No hand")).expect("Failed to parse hand");
            let bid = split_line
                .next()
                .expect("No bid")
                .parse::<Num>()
                .expect("Failed to parse bid");
            (hand, bid)
        })
        .collect();
    hands.sort();
    for (hand, bid) in hands.iter().rev() {
        println!("{} {} {:#?}", hand, bid, hand.get_type());
    }
    return hands
        .iter()
        .enumerate()
        .map(|(idx, (_, bid))| -> Num { (idx as Num + 1) * bid })
        .sum();
}
