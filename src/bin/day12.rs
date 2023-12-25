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
            get_possibility_count(&fountains, &groups)
        })
        .sum()
}

fn get_possibility_count(fountains: &Fountains, groups: &Groups) -> Num {
    // This extra fountain is needed to extend iteration because our window size is one larger to include a terminating "good" fountain.
    let mut fountains_extended: Fountains = fountains.clone();
    fountains_extended.push(Good);
    get_possibility_count_helper(&fountains_extended, &groups)
}

fn get_possibility_count_helper(fountains: &[Fountain], groups: &[u8]) -> Num {
    if groups.len() == 0 {
        // It's our duty to make sure there are no more broken fountains (that would require more groups declared)
        if fountains.iter().all(|fountain| *fountain != Broken) {
            return 1;
        } else {
            return 0;
        }
    }
    let mut counter = 0;
    let current_group_size = groups[0] as usize;
    let window_size = current_group_size + 1;
    for (idx, window) in fountains.windows(window_size).enumerate() {
        let mut window_permutations = 0;
        let can_group_fit_window = window[0..current_group_size]
            .iter()
            .all(|fountain| *fountain != Good);
        let is_group_terminated = window[current_group_size] != Broken;
        if can_group_fit_window && is_group_terminated {
            window_permutations += 1;
        }

        let (_, remainder_fountains) = fountains.split_at(idx + window_size);
        let (_, remainder_groups) = groups.split_at(1);
        let remainder_permutations =
            get_possibility_count_helper(remainder_fountains, remainder_groups);

        counter += window_permutations * remainder_permutations;

        // Here, we must check to see if the group is anchored here.
        // That is, there's a known broken fountain here so we can't iterate futher and consider it part of the same group.
        if window[0] == Broken {
            break;
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

    #[test]
    fn test_get_possibility_count() {
        let example_pairs: Vec<(&str, Num)> = vec![
            ("???.### 1,1,3", 1),
            (".??..??...?##. 1,1,3", 4),
            ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
            ("????.#...#... 4,1,1", 1),
            ("????.######..#####. 1,6,5", 4),
            ("?###???????? 3,2,1", 10),
            // Less groups present than possible
            ("#.# 1", 0),
        ];
        for (input_line, expected) in example_pairs.iter() {
            let input = parse_fountain_line(&input_line);
            let actual = get_possibility_count(&input.0, &input.1);
            println!(">> {}", &input_line);
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
