use advent_of_code_2023::day12::Fountain::*;
use advent_of_code_2023::day12::*;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap());
    let result = day12(lines);
    println!("{}", result);
}

fn day12(mut lines: impl Iterator<Item = impl AsRef<str>>) -> Num {
    lines
        .map(|line| {
            let (fountains, groups) = parse_fountain_line(line.as_ref());
            get_possibility_count(&fountains, &mut groups.iter())
        })
        .sum()
}

fn get_possibility_count<'a>(
    fountains: &[Fountain],
    mut groups: &mut impl Iterator<Item = &'a u8>,
) -> Num {
    let mut counter = 0;
    // brute force it
    if let Some(group_len) = &mut groups.next() {
        assert!(*group_len > &0);
        // march across the locations
        let mut start_offset = 0;

        let mut fountains_iter = fountains.iter().enumerate();
        while let Some((attempt_start_idx, is_known)) = fountains_iter.find_map(|(idx, f)| match *f
        {
            Good => None,
            Broken => Some((idx, true)),
            Unknown => Some((idx, false)),
        }) {
            // attempt to construct a group at this location
            // when we find a known broken fountain, we MUST include it, so we can't begin additional attempts beyond this location.
            let is_final_attempt = is_known;
            let mut remaining_quota = *group_len - 1;
            let quota_reached = (|| {
                while remaining_quota > 0 {
                    match fountains_iter.next() {
                        None => return false,
                        Some((_, status)) => match status {
                            Good => return false,
                            Broken => remaining_quota -= 1,
                            Unknown => remaining_quota -= 1,
                        },
                    }
                }
                return true;
            })();
            // Not done yet. We need to ensure the next slot doesn't extend us beyond the quota!
            let attempt_is_valid = match fountains_iter.next() {
                None => true, // End of input; valid
                Some((_, status)) => {
                    match status {
                        Good => true,    // valid
                        Unknown => true, // valid
                        Broken => false, // invalid
                    }
                }
            };

            if attempt_is_valid {
                // account for the extra Good item
                let remainder_start = attempt_start_idx + (**group_len as usize) + 1;
                if remainder_start < fountains.len() {
                    // recursion
                    counter += get_possibility_count(&fountains[remainder_start..], &mut groups)
                }
            }

            if is_final_attempt {
                break;
            }
        }
    } else {
        // There should be no more groups, so it's our respnsibility to make sure there are no explicit Broken fountains in the rest of the string
        if fountains.iter().find(|f| Broken == **f).is_none() {
            counter += 1;
        }
    }
    return counter;
}

enum FountainSearchState {
    SearchingForGroup,
    InGroup(u8),
    GroupCompleted,
}

use FountainSearchState::*;

fn is_arrangement_valid(fountains: &Fountains, groups: &Groups) -> bool {
    let mut fountain_iter = fountains.iter();
    for group_size in groups.iter() {
        assert!(*group_size > 0);
        let mut search_state = FountainSearchState::SearchingForGroup;
        for fountain_status in &mut fountain_iter {
            search_state = match search_state {
                SearchingForGroup => match fountain_status {
                    Broken => InGroup(1),
                    Good => SearchingForGroup,
                    Unknown => panic!("Only fully-defined arrangements can be validated"),
                },
                InGroup(curr_count) => {
                    match fountain_status {
                        Broken => InGroup(curr_count + 1),
                        Good => {
                            if curr_count == *group_size {
                                // We've found a complete valid run, check the next one.
                                GroupCompleted
                            } else {
                                // Run ended too soon.
                                return false;
                            }
                        }
                        Unknown => panic!("Only fully-defined arrangements can be validated"),
                    }
                }
                GroupCompleted => unreachable!(),
            };
            if let GroupCompleted = search_state {
                // We've found a complete valid run, check the next one.
                break;
            }
        }
        // Ran out of fountain input or completed group successfully
        match search_state {
            SearchingForGroup => return false, // Never found the first broken fountain we needed.
            GroupCompleted => continue, // We've found a complete valid run, check the next one.
            InGroup(curr_count) => {
                if curr_count == *group_size {
                    // We've found a complete valid run, check the next one.
                    continue;
                } else {
                    // Run ended too soon.
                    return false;
                }
            }
        }
    }
    // All of the groups were valid.
    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let example = vec![
            "#.#.### 1,1,3",
            ".#...#....###. 1,1,3",
            ".#.###.#.###### 1,3,1,6",
            "####.#...#... 4,1,1",
            "#....######..#####. 1,6,5",
            ".###.##....# 3,2,1",
        ];
        let actual = day12(example.iter());
        assert_eq!(actual, 21);
    }

    #[test]
    fn test_get_possibility_count() {
        let example_pairs: Vec<(&str, Num)> = vec![
            ("???.### 1,1,3", 1),
            (".??..??...?##. 1,1,3", 4),
            ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
            ("????.#...#... 4,1,1", 1),
            ("????.######..#####. 1,6,5", 4),
            ("?###???????? 3,2,1", 10),
        ];
        for (input_line, expected) in example_pairs.iter() {
            let input = parse_fountain_line(&input_line);
            let actual = get_possibility_count(&input.0, &mut input.1.iter());
            assert_eq!(actual, *expected);
        }
    }

    #[test]
    fn test_is_arrangement_valid() {
        let example_pairs: Vec<(&str, bool)> = vec![
            ("#.#.### 1,1,3", true),
            (".#...#....###. 1,1,3", true),
            (".#.###.#.###### 1,3,1,6", true),
            ("####.#...#... 4,1,1", true),
            ("#....######..#####. 1,6,5", true),
            (".###.##....# 3,2,1", true),
            ("..##.. 1", false),
            ("..##.. 3", false),
            ("..##..##.. 2, 1", false),
            ("..##..##.. 2, 3", false),
            ("..##.. 2, 1", false),
            ("..##.. 2, 3", false),
        ];
        for (input_line, expected) in example_pairs.iter() {
            let input = parse_fountain_line(&input_line);
            let actual = is_arrangement_valid(&input.0, &input.1);
            println!(">> {}", &input_line);
            assert_eq!(actual, *expected);
        }
    }
}
