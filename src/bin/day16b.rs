use advent_of_code_2023::{cursor_grid::*, day16::*};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.expect("IO Error"));
    let result = day16b(lines);
    println!("{}", result);
}

fn day16b(lines: impl Iterator<Item = impl AsRef<str>>) -> Num {
    let grid = parse_grid(lines);
    let start_cursors = get_initial_cursors(&grid);
    start_cursors
        .map(|start_cursor| count_energized_tiles(start_cursor, &grid))
        .max()
        .unwrap()
}

fn get_initial_cursors<T>(grid: &Grid<T>) -> impl Iterator<Item = Cursor> {
    let h = grid.rows.len() as i32;
    let w = grid.rows[0].len() as i32;
    let left_cursors = (0..h).map(|y| ((y, 0), Direction::Right));
    let right_cursors = (0..h).map(move |y| ((y, w - 1), Direction::Left));
    let top_cursors = (0..w).map(|x| ((0, x), Direction::Down));
    let bottom_cursors = (0..w as i32).map(move |x| ((h - 1, x), Direction::Up));
    left_cursors
        .chain(right_cursors)
        .chain(top_cursors)
        .chain(bottom_cursors)
}
