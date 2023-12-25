use advent_of_code_2023::day12::*;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap());
    let result = day12b(lines);
    println!("{}", result);
}

fn day12b(lines: impl Iterator<Item = impl AsRef<str>>) -> Num {
    lines
        .map(|line| {
            let (mut fountains, mut groups) = parse_fountain_line(line.as_ref());
            quintuple_inputs(&mut fountains, &mut groups);
            get_possibility_count(&fountains, &groups)
        })
        .sum()
}

fn quintuple_inputs(fountains: &mut Fountains, groups: &mut Groups) {
    let mut fountains2 = fountains.clone();
    for _ in 0..4 {
        fountains2.push(Fountain::Unknown);
        fountains2.extend(fountains.clone().into_iter());
    }
    *fountains = fountains2;
    *groups = groups.repeat(5);
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
        let actual = day12b(example.iter());
        assert_eq!(actual, 525152);
    }
}
