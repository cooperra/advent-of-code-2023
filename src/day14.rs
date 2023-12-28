use std::collections::HashMap;

type Num = usize;
type Row = Vec<Option<Rock>>;

pub fn day14(lines: impl Iterator<Item = impl AsRef<str>>) -> Num {
    let parsed = parse(lines);
    let shifted = shift_up(parsed);
    weigh(shifted.into_iter())
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

fn shift_up(rows: impl Iterator<Item = Row>) -> Vec<Row> {
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

    #[test]
    fn test_example() {
        let input = EXAMPLE;
        let result = day14(input.into_iter());
        assert_eq!(result, 136);
    }
}
