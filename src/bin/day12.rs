use advent_of_code_2023::day12::*;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap());
    let result = day12(lines);
    println!("{}", result);
}

fn day12(lines: impl Iterator<Item = impl AsRef<str>>) -> Num {
    lines
        .map(|line| {
            let (fountains, groups) = parse_fountain_line(line.as_ref());
            get_possibility_count(&fountains, &groups)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let example = vec![
            "???.### 1,1,3",
            ".??..??...?##. 1,1,3",
            "?#?#?#?#?#?#?#? 1,3,1,6",
            "????.#...#... 4,1,1",
            "????.######..#####. 1,6,5",
            "?###???????? 3,2,1",
        ];
        let actual = day12(example.iter());
        assert_eq!(actual, 21);
    }
}
