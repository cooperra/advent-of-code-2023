use crate::zip_many::zip_many;
use std::collections::HashMap;

type Num = usize;
type Row = Vec<Option<Rock>>;

pub fn day14(lines: impl Iterator<Item = impl AsRef<str>>) -> Num {
    let parsed = parse(lines);
    let shifted = shift_up(parsed);
    weigh(shifted.into_iter())
}

pub fn day14b(lines: impl Iterator<Item = impl AsRef<str>>, spin_cycles: Num) -> Num {
    let parsed = parse(lines);
    let mut rotated: Vec<Row> = parsed.collect();
    for _ in 0..spin_cycles {
        rotated = do_cycle(rotated);
    }
    weigh(rotated.into_iter())
}

fn do_cycle(grid: Vec<Row>) -> Vec<Row> {
    let mut rotated = grid;
    for _ in 0..4 {
        rotated = do_quarter_cycle(rotated);
    }
    rotated
}

fn do_quarter_cycle(grid: Vec<Row>) -> Vec<Row> {
    let shifted = shift_up(grid);
    let rotated = rotate_ccw(shifted).collect();
    rotated
}

fn parse(lines: impl Iterator<Item = impl AsRef<str>>) -> impl Iterator<Item = Row> {
    lines.map(parse_line)
}

fn parse_line(line: impl AsRef<str>) -> Row {
    line.as_ref()
        .chars()
        .map(|c| match c {
            'O' => Some(Rock::Round),
            '#' => Some(Rock::Square),
            '.' => None,
            _ => panic!("Not a valid character"),
        })
        .collect()
}

fn shift_up(rows: impl IntoIterator<Item = Row>) -> Vec<Row> {
    let mut grid: Vec<Row> = Vec::new();
    let mut current_blockers_per_col = HashMap::<usize, usize>::new();
    for (row_idx, mut row) in rows.into_iter().enumerate() {
        for (col_idx, item) in row.iter_mut().enumerate() {
            match item {
                Some(Rock::Square) => {
                    // The new rock is a blocker.
                    current_blockers_per_col.insert(col_idx, row_idx);
                }
                Some(Rock::Round) => {
                    // Slide this rock down if possible.
                    let destination_row_idx;
                    if let Some(current_blocker) = current_blockers_per_col.get(&col_idx) {
                        destination_row_idx = current_blocker + 1;
                    } else {
                        destination_row_idx = 0;
                    }

                    // Avoid moving into the current row because it isn't in the grid yet.
                    // (And it's already there.)
                    if destination_row_idx != row_idx {
                        // Move from here
                        *item = None;
                        // To here
                        grid[destination_row_idx][col_idx] = Some(Rock::Round);
                    }

                    // The new rock is a blocker.
                    current_blockers_per_col.insert(col_idx, destination_row_idx);
                }
                None => (),
            };
        }
        grid.push(row);
    }
    grid
}

fn weigh(grid: impl ExactSizeIterator<Item = Row>) -> Num {
    let row_count = grid.len();
    let row_weights = grid.enumerate().map(|(row_idx, row)| {
        let row_round_rock_count = row
            .into_iter()
            .filter(|item| *item == Some(Rock::Round))
            .count();
        let weight_multiplier = row_count - row_idx;
        row_round_rock_count * weight_multiplier
    });
    row_weights.sum()
}

fn rotate_ccw(grid: Vec<Row>) -> impl Iterator<Item = Row> {
    zip_many(grid.into_iter().rev())
}

#[derive(Eq, PartialEq, Debug)]
enum Rock {
    Round,
    Square,
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &[&str] = &[
        "O....#....",
        "O.OO#....#",
        ".....##...",
        "OO.#O....O",
        ".O.....O#.",
        "O.#..O.#.#",
        "..O..#O..O",
        ".......O..",
        "#....###..",
        "#OO..#....",
    ];

    const EXAMPLE_1CYCLE: &[&str] = &[
        ".....#....",
        "....#...O#",
        "...OO##...",
        ".OO#......",
        ".....OOO#.",
        ".O#...O#.#",
        "....O#....",
        "......OOOO",
        "#...O###..",
        "#..OO#....",
    ];

    const EXAMPLE_2CYCLE: &[&str] = &[
        ".....#....",
        "....#...O#",
        ".....##...",
        "..O#......",
        ".....OOO#.",
        ".O#...O#.#",
        "....O#...O",
        ".......OOO",
        "#..OO###..",
        "#.OOO#...O",
    ];

    const EXAMPLE_3CYCLE: &[&str] = &[
        ".....#....",
        "....#...O#",
        ".....##...",
        "..O#......",
        ".....OOO#.",
        ".O#...O#.#",
        "....O#...O",
        ".......OOO",
        "#...O###.O",
        "#.OOO#...O",
    ];

    #[test]
    fn test_example() {
        let input = EXAMPLE;
        let result = day14(input.into_iter());
        assert_eq!(result, 136);
    }

    #[test]
    fn test_1cycle() {
        let input = parse(EXAMPLE.into_iter()).collect();
        let expected: Vec<Row> = parse(EXAMPLE_1CYCLE.into_iter()).collect();
        let result = do_cycle(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2cycle() {
        let input = parse(EXAMPLE.into_iter()).collect();
        let expected: Vec<Row> = parse(EXAMPLE_2CYCLE.into_iter()).collect();
        let result = do_cycle(do_cycle(input));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3cycle() {
        let input = parse(EXAMPLE.into_iter()).collect();
        let expected: Vec<Row> = parse(EXAMPLE_3CYCLE.into_iter()).collect();
        let result = do_cycle(do_cycle(do_cycle(input)));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_b_short() {
        let input = EXAMPLE;
        let expected = weigh(
            parse(EXAMPLE_3CYCLE.into_iter())
                .collect::<Vec<_>>()
                .into_iter(),
        );
        let result = day14b(input.into_iter(), 3);
        assert_eq!(result, expected);
    }

    //#[test]
    fn test_example_b() {
        let input = EXAMPLE;
        let result = day14b(input.into_iter(), 1000000000);
        assert_eq!(result, 64);
    }
}
