use itertools::Itertools;
use std::io::{self, BufRead};

use advent_of_code_2023::cursor_grid::{manhattan, Coord, Grid};

type Num = u32;
type Universe = Vec<Coord>;
type Node = bool;

fn main() {
    let stdin = io::stdin().lock();
    let lines = stdin.lines().map(|l| l.expect("IO Error"));
    let result = day11(lines);
    println!("{}", result);
}

fn day11(lines: impl Iterator<Item = impl AsRef<str>>) -> Num {
    let mut universe = parse_grid(lines);
    expand_universe(&mut universe);
    sum_distances(&universe)
}

fn parse_grid(lines: impl Iterator<Item = impl AsRef<str>>) -> Universe {
    let mut universe = Vec::new();
    for (row_idx, line) in lines.enumerate() {
        let row_galaxies = line
            .as_ref()
            .chars()
            .enumerate()
            .filter_map(|(col_idx, c)| (c == '#').then_some((row_idx as i32, col_idx as i32)));
        universe.extend(row_galaxies);
    }
    universe
}

fn expand_universe(mut universe: &mut Universe) {
    let multiplier = 2;
    replicate_empty_rows(&mut universe, multiplier);
    replicate_empty_columns(&mut universe, multiplier);
}

fn replicate_empty_columns(universe: &mut Universe, multiplier: u32) {
    universe.sort_by_key(|(_, col_idx)| *col_idx);
    let mut col_shift: i32 = 0;
    let mut current_col: i32 = 0;
    for (_, galaxy_col) in universe.iter_mut() {
        let col_delta: i32 = *galaxy_col - current_col;
        if col_delta > 0 {
            current_col = *galaxy_col;
            let empty_col_count = col_delta - 1;
            // When col_delta is 1, this should have no effect, because there are no empty cols.
            let empty_col_count_expanded = empty_col_count * multiplier as i32;
            col_shift += empty_col_count_expanded - empty_col_count;
        }
        *galaxy_col += col_shift;
    }
}

fn replicate_empty_rows(universe: &mut Universe, multiplier: u32) {
    // Assume already sorted by row
    let mut row_shift: i32 = 0;
    let mut current_row: i32 = 0;
    for (galaxy_row, _) in universe.iter_mut() {
        let row_delta: i32 = *galaxy_row - current_row;
        if row_delta > 0 {
            current_row = *galaxy_row;
            let empty_row_count = row_delta - 1;
            // When row_delta is 1, this should have no effect, because there are no empty rows.
            let empty_row_count_expanded = empty_row_count * multiplier as i32;
            row_shift += empty_row_count_expanded - empty_row_count;
        }
        *galaxy_row += row_shift;
    }
}

fn sum_distances(universe: &Universe) -> Num {
    let galaxy_pairs = galaxy_pairs(&universe);
    galaxy_pairs
        .into_iter()
        .map(|pair| manhattan(pair[0], pair[1]))
        .sum()
}

fn galaxy_pairs(universe: &Universe) -> impl Iterator<Item = Vec<Coord>> + Sized + '_ {
    let galaxy_positions = locate_galaxies(universe);
    galaxy_positions.combinations(2)
}

fn locate_galaxies(universe: &Universe) -> impl Iterator<Item = Coord> + '_ {
    universe.clone().into_iter()
}
