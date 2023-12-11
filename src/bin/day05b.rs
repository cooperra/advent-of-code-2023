use advent_of_code_2023::day05::*;
use std::io::{self, BufRead};

type Num = u64;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap());
    let result = day05b(lines);
    println!("{}", result);
}

pub fn day05b(mut lines: impl Iterator<Item = impl AsRef<str>>) -> Num {
    let almanac = Almanac::parse(&mut lines);

    let seed_ranges = almanac.seed_ranges();
    //for seed in seed_ranges.iter() {
    //    assert!(seed.start < seed.end);
    //}
    //assert_eq!(seed_ranges.len() * 2, almanac.seeds.len());

    let locations = almanac
        .maps
        .iter()
        .fold(seed_ranges, |curr_ranges, map| map.map_ranges(curr_ranges));

    // Choose min location
    return locations
        .into_iter()
        .map(|r| r.start)
        .reduce(Num::min)
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    //use advent_of_code_2023::day05::get_example;

    #[test]
    fn example() {
        let mut lines = get_example().into_iter();
        let actual = day05b(lines);
        assert_eq!(actual, 46);
    }
}
