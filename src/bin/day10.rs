use advent_of_code_2023::{cursor_grid::*, day10::*};
use std::io::{self, BufRead};

type Num = u32;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.expect("IO Error"));
    let result = day10(lines);
    println!("{}", result);
}

fn day10(lines: impl Iterator<Item = impl AsRef<str>>) -> Num {
    let (grid, start_pos) = parse_grid(lines);
    let mut cursors = get_initial_cursors(&start_pos, &grid);
    assert_eq!(2, cursors.len());
    let mut iter_count = 0;
    loop {
        iter_count += 1;
        cursors = cursors
            .into_iter()
            .map(|cursor: Cursor| cursor.next(&grid))
            .collect();
        let ref some_pos = cursors[1].0;
        if cursors.iter().all(|cursor: &Cursor| some_pos == &cursor.0) {
            return iter_count;
        }
    }
}
