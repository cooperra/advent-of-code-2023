use std::collections::HashSet;

use itertools::Itertools;

type Num = u32;

pub fn day13(lines: impl Iterator<Item = impl AsRef<str>>) -> Num {
    let mut sum = 0;
    let mut lines2 = lines.peekable();
    while lines2.peek().is_some() {
        let chunk = collect_chunk(&mut lines2);
        sum += process_chunk(chunk);
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

fn process_chunk(lines: Vec<impl AsRef<str>>) -> Num {
    let chars = lines
        .iter()
        .map(|x| x.as_ref().chars().collect_vec())
        .collect_vec();
    100 * find_horizontal_reflection(&chars).unwrap_or(0)
        + find_vertical_reflection(&chars).unwrap_or(0)
}

fn find_horizontal_reflection(chars: &Vec<Vec<char>>) -> Option<Num> {
    let transposed =
        zip_many(chars.iter().map(|x| x.iter().map(|x| x.clone())).collect()).collect_vec();
    find_vertical_reflection(&transposed)
}

fn find_vertical_reflection(chars: &Vec<Vec<char>>) -> Option<Num> {
    let reflections_per_row = chars.iter().map(|row| -> HashSet<usize> {
        let starting_points = row
            .windows(2)
            .enumerate()
            .filter_map(|(idx, pair)| (pair[0] == pair[1]).then_some(idx));
        // Check for complete reflection (within this row only)
        let row_reflections = starting_points
            .filter(|idx| {
                let mut left_idx = *idx;
                let mut right_idx = *idx + 1;
                while right_idx < row.len() {
                    if row[left_idx] != row[right_idx] {
                        return false;
                    }
                    if left_idx == 0 {
                        // Prevent underflow
                        break;
                    }
                    left_idx -= 1;
                    right_idx += 1;
                }
                return true;
            })
            .collect();
        row_reflections
    });
    let reflection_set: HashSet<usize> = reflections_per_row
        .reduce(|set1, set2| set1.intersection(&set2).map(|x| *x).collect())
        .unwrap_or_default();

    reflection_set.iter().next().map(|x| (*x + 1) as Num)
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

    #[test]
    fn example() {
        let input = vec![
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
        let result = day13(input.into_iter());
        assert_eq!(result, 405);
    }
}
