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
    //println!("{:#?}", almanac);
    //panic!();
    let mut locations = Vec::<Num>::new();
    let seed_ranges = almanac
        .seeds
        .chunks_exact(2)
        .map(|slice: &[u64]| {
            let start = slice[0];
            let end = start + slice[1];
            start..end
        })
        .collect::<Vec<_>>();
    //println!("seed_ranges done");
    //let seed_iters = seed_ranges
    //    .iter()
    //    .map(|r| r.into_iter())
    //    .collect::<Vec<_>>();
    let seeds = seed_ranges.into_iter().flatten();
    //println!("seeds done");
    for seed in seeds {
        //println!("seed {}", seed);
        let loc = almanac
            .maps
            .iter()
            .fold(seed, |curr: Num, map| map.map(curr));
        locations.push(loc);
        //println!("pushed loc");
    }

    // Choose min location
    //println!("locations {:#?}", &locations);
    return locations.into_iter().reduce(Num::min).unwrap();
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
