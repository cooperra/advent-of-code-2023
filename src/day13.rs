use std::collections::HashMap;

use itertools::Itertools;

type Num = u32;

pub fn day13(lines: impl Iterator<Item = impl AsRef<str>>, accepted_error_count: u8) -> Num {
    let mut sum = 0;
    let mut lines2 = lines.peekable();
    while lines2.peek().is_some() {
        let chunk = collect_chunk(&mut lines2);
        sum += process_chunk(chunk, accepted_error_count);
    }
    sum
}

fn collect_chunk<S: AsRef<str>>(lines: &mut impl Iterator<Item = S>) -> Vec<S> {
    let mut buffer = Vec::new();
    for line in lines {
        if line.as_ref() == "" {
            break;
        }
        buffer.push(line);
    }
    buffer
}

fn process_chunk(lines: Vec<impl AsRef<str>>, accepted_error_count: u8) -> Num {
    let chars = lines
        .iter()
        .map(|x| x.as_ref().chars().collect_vec())
        .collect_vec();
    100 * find_horizontal_reflection(&chars, accepted_error_count).unwrap_or(0)
        + find_vertical_reflection(&chars, accepted_error_count).unwrap_or(0)
}

fn find_horizontal_reflection(chars: &Vec<Vec<char>>, accepted_error_count: u8) -> Option<Num> {
    let transposed =
        zip_many(chars.iter().map(|x| x.iter().map(|x| x.clone())).collect()).collect_vec();
    find_vertical_reflection(&transposed, accepted_error_count)
}

fn find_vertical_reflection(chars: &Vec<Vec<char>>, accepted_error_count: u8) -> Option<Num> {
    let reflections_per_row = chars.iter().map(|row| -> HashMap<usize, u8> {
        let row_reflections = row
            .windows(2)
            .enumerate()
            // Check for complete reflection (within this row only)
            .filter_map(|(idx, _)| {
                let mut row_errors = 0;
                let mut left_idx = idx;
                let mut right_idx = idx + 1;
                while right_idx < row.len() {
                    if row[left_idx] != row[right_idx] {
                        row_errors += 1;
                    }
                    if row_errors > accepted_error_count {
                        return None;
                    }
                    if left_idx == 0 {
                        // Prevent underflow
                        break;
                    }
                    left_idx -= 1;
                    right_idx += 1;
                }
                return Some((idx, row_errors));
            })
            .collect();
        row_reflections
    });
    let reflection_set = reflections_per_row
        .reduce(|row1, row2| {
            let accumulated_errors = row1
                .into_iter()
                .filter_map(|(idx, errors1)| {
                    row2.get(&idx)
                        .map(|errors2| {
                            let errors = errors1 + *errors2;
                            (idx, errors)
                        })
                        .filter(|(_, errors)| *errors <= accepted_error_count)
                })
                .collect();
            accumulated_errors
        })
        .unwrap_or_default();

    reflection_set
        .iter()
        .filter_map(|(idx, errors)| (*errors == accepted_error_count).then_some((*idx + 1) as Num))
        .next()
}

fn zip_many<I, O, T>(iters: Vec<I>) -> ZipMany<O, T>
where
    I: IntoIterator<Item = T, IntoIter = O>,
    O: Iterator<Item = T>,
{
    ZipMany {
        iters: iters.into_iter().map(|item| item.into_iter()).collect(),
    }
}

struct ZipMany<I, T>
where
    I: Iterator<Item = T>,
{
    iters: Vec<I>,
}

impl<I, T> Iterator for ZipMany<I, T>
where
    I: Iterator<Item = T>,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut nexts = Vec::new();
        for iter in self.iters.iter_mut() {
            if let Some(next) = iter.next() {
                nexts.push(next);
            } else {
                return None;
            }
        }
        return Some(nexts);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &[&str] = &[
        "#.##..##.",
        "..#.##.#.",
        "##......#",
        "##......#",
        "..#.##.#.",
        "..##..##.",
        "#.#.##.#.",
        "",
        "#...##..#",
        "#....#..#",
        "..##..###",
        "#####.##.",
        "#####.##.",
        "..##..###",
        "#....#..#",
    ];

    #[test]
    fn test_example() {
        let input = EXAMPLE;
        let result = day13(input.into_iter(), 0);
        assert_eq!(result, 405);
    }

    #[test]
    fn test_example_b() {
        let input = EXAMPLE;
        let result = day13(input.into_iter(), 1);
        assert_eq!(result, 400);
    }
}
