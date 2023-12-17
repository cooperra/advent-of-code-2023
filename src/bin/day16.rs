use advent_of_code_2023::{cursor_grid::*, day16::*};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.expect("IO Error"));
    let result = day16(lines);
    println!("{}", result);
}

fn day16(lines: impl Iterator<Item = impl AsRef<str>>) -> Num {
    let grid = parse_grid(lines);
    let start_cursor = ((0, 0), Direction::Right);
    count_energized_tiles(start_cursor, &grid)
}
