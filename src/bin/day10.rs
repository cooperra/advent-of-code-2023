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
    let pipe_loop_iter = PipeLoopIterator::new(start_pos, &grid);
    pipe_loop_iter.count() as Num / 2
}
