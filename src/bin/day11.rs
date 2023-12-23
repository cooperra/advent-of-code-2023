use itertools::Itertools;
use std::io::{self, BufRead};

use advent_of_code_2023::cursor_grid::{manhattan, Coord, Grid};

type Num = u32;
type Universe = Grid<Node>;
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
    let mut grid = Grid::new();
    for line in lines {
        let row: Vec<Node> = line.as_ref().chars().map(|c| c == '#').collect();
        grid.rows.push(row);
    }
    grid
}

fn expand_universe(mut universe: &mut Universe) {
    // expand cols
    // Assume all rows same length
    duplicate_column_if(&mut universe, |c| c.into_iter().all(|x| !*x));

    // expand rows
    duplicate_item_if(&mut universe.rows, |r: &Vec<Node>| r.iter().all(|x| !*x));
}

fn duplicate_column_if<T>(grid: &mut Grid<T>, pred: fn(Vec<&T>) -> bool)
where
    T: Clone,
{
    let matching_indexes: Vec<_> = grid
        .col_iter()
        .enumerate()
        .filter_map(|(idx, item)| pred(item).then_some(idx))
        .collect();
    // Reverse ensures we don't invalidate our remaining indexes.
    for idx in matching_indexes.into_iter().rev() {
        let new_col: Vec<_> = grid
            .get_col(idx)
            .into_iter()
            .map(|item| item.clone())
            .collect();
        grid.insert_col(idx, new_col);
    }
}

fn duplicate_item_if<T>(v: &mut Vec<T>, pred: fn(&T) -> bool)
where
    T: Clone,
{
    // There's gotta be a better way (a crate for mutating iterators), but the internet stubbornly disagrees.
    let mut idx = 0;
    while idx < v.len() {
        let ref item = v[idx];
        if pred(item) {
            v.insert(idx, item.clone());
            idx += 1;
        }
        idx += 1;
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
    universe
        .iter()
        .zip(universe.positions())
        .filter_map(|(is_galaxy, pos)| is_galaxy.then_some(pos))
}
