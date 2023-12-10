use regex::Regex;
use std::collections::HashSet;

pub fn day04(lines: impl Iterator<Item = impl AsRef<str>>) -> u32 {
    let mut sum = 0;
    for line in lines {
        sum += score_for_line(line.as_ref());
    }
    return sum;
}

fn score_for_line(line: &str) -> u32 {
    let card = Card::from_line(&line);
    card.get_score()
}

#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    id: u32,
    win_nums: HashSet<u8>,
    your_nums: HashSet<u8>,
}

impl Card {
    pub fn new(id: u32, win_nums: HashSet<u8>, your_nums: HashSet<u8>) -> Self {
        Self {
            id,
            win_nums,
            your_nums,
        }
    }

    pub fn from_line(line: &str) -> Self {
        let id: u32;
        let mut win_nums: HashSet<u8> = HashSet::new();
        let mut your_nums: HashSet<u8> = HashSet::new();
        let whole_re =
            Regex::new(r"Card\s+([0-9]+):(?:\s+([0-9]+))*\s*(\|)(?:\s+([0-9]+))*").unwrap();
        //let capture = whole_re.captures_iter(&line).next().unwrap();
        //let mut subcaptures = capture.iter().map(|m| m.unwrap().as_str()).skip(1);
        whole_re
            .captures_iter(&line)
            .next()
            .expect("Invalid Card string");
        let re = Regex::new(r"([0-9]+|\|)").unwrap();
        let mut captures = re.find_iter(&line).map(|m| m.as_str());
        id = captures.next().unwrap().parse().unwrap();
        for find in &mut captures {
            if "|" == find {
                break;
            }
            let num: u8 = find.parse().unwrap();
            win_nums.insert(num);
        }
        for find in &mut captures {
            let num: u8 = find.parse().unwrap();
            your_nums.insert(num);
        }
        Self::new(id, win_nums, your_nums)
    }

    pub fn get_matches(self: &Self) -> u8 {
        self.win_nums.intersection(&self.your_nums).count() as u8
    }

    pub fn get_score(self: &Self) -> u32 {
        match self.get_matches() as u32 {
            0 => 0,
            n => 2_u32.pow(n - 1),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_parsing() {
        let line: &str = "Card 1: 2 30 4 | 5 6 7";
        let actual = Card::from_line(&line);
        let expected = Card::new(
            1,
            HashSet::<u8>::from([2, 30, 4]),
            HashSet::<u8>::from([5, 6, 7]),
        );
        assert_eq!(&expected, &actual);
    }

    #[test]
    fn example() {
        let lines = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];
        let result = day04(&mut lines.into_iter());
        assert_eq!(result, 13);
    }

    #[test]
    fn example_card_scores() {
        let lines = vec![
            ("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8),
            ("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2),
            ("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2),
            ("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1),
            ("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0),
            ("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0),
        ];
        for (line, expected_score) in lines.into_iter() {
            let result = score_for_line(&line);
            assert_eq!(result, expected_score);
        }
    }
}
