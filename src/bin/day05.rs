use advent_of_code_2023::day05::*;
use std::io::{self, BufRead};

type Num = u64;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap());
    let result = day05(lines);
    println!("{}", result);
}

pub fn day05(mut lines: impl Iterator<Item = impl AsRef<str>>) -> Num {
    let almanac = Almanac::parse(&mut lines);
    let mut locations = Vec::<Num>::new();
    for seed in almanac.seeds {
        let loc = almanac
            .maps
            .iter()
            .fold(seed, |curr: Num, map| map.map(curr));
        locations.push(loc);
    }

    // Choose min location
    //println!("locations {:#?}", &locations);
    return locations.into_iter().reduce(Num::min).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2023::day05::get_example;

    #[test]
    fn example() {
        let mut lines = get_example().into_iter();
        let actual = day05(lines);
        assert_eq!(actual, 35);
    }
}
