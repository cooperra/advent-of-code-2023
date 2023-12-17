use advent_of_code_2023::{cursor_grid::*, day16::*};
use std::{
    collections::HashSet,
    io::{self, BufRead},
};

type Num = u32;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.expect("IO Error"));
    let result = day16(lines);
    println!("{}", result);
}

fn day16(lines: impl Iterator<Item = impl AsRef<str>>) -> Num {
    let grid = parse_grid(lines);
    let mut energy_map: HashSet<Coord> = HashSet::new();
    let mut cursors = vec![((0, 0), Direction::Right)];
    loop {
        cursors = cursors
            .into_iter()
            .flat_map(|(pos, dir): Cursor| {
                let node = grid.get(&pos);
                let is_energized_splitter = match node {
                    Some(MirrorSplitter::Splitter(_)) => energy_map.contains(&pos),
                    _ => false,
                };
                energy_map.insert(pos);
                if is_energized_splitter {
                    vec![]
                } else {
                    next_cursors(&(pos, dir), node)
                }
            })
            .filter(|cursor| grid.is_within_bounds(&cursor.0))
            .collect();
        if cursors.is_empty() {
            break;
        }
    }
    return energy_map.len() as Num;
}

fn next_cursors(cursor: &Cursor, node: &Node) -> Vec<Cursor> {
    let next_dirs = route_light(*node, cursor.1);
    next_dirs.iter().map(|d| (cursor.0 + *d, *d)).collect()
}
