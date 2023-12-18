use advent_of_code_2023::cursor_grid::*;
use advent_of_code_2023::linked_list::LinkedList;
use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{self, BufRead},
};

type Num = u32;
#[derive(PartialEq, Eq)]
struct HeapEntry(u32, u32, Direction, u8, LinkedList<Coord>);

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.expect("IO Error"));
    let result = day17(lines);
    println!("{}", result);
}

fn day17(lines: impl Iterator<Item = impl AsRef<str>>) -> Num {
    let grid = parse_grid(lines);
    // let start_cursors = get_initial_cursors();
    // start_cursors
    //     .map(|start_cursor| count_energized_tiles(start_cursor, &grid))
    //     .max()
    //     .unwrap()
    let start_pos = (0, 0);
    let dest_pos = (grid.rows.len() as i32 - 1, grid.rows[0].len() as i32);
    let mut candidate_paths: BinaryHeap<Reverse<HeapEntry>> = [Reverse(HeapEntry(
        manhattan(start_pos, dest_pos),    // heuristic
        0,                                 // path_cost
        Direction::Right,                  // current_direction
        0,                                 // direction_repeats
        [start_pos].into_iter().collect(), // path_nodes
    ))]
    .into_iter()
    .collect();
    loop {
        let Reverse(HeapEntry(_, path_cost, current_direction, direction_repeats, path_nodes)) =
            candidate_paths.pop().unwrap();
        let current_pos = path_nodes.head().unwrap();
        if *current_pos == dest_pos {
            return path_cost;
        }
        let nexts = neighbor_steps(*current_pos).filter_map(|(pos, dir)| {
            if !grid.is_within_bounds(&pos) {
                return None;
            }
            if path_nodes.contains(&pos) {
                return None;
            }
            let mut new_repeats = direction_repeats;
            if dir == current_direction {
                if direction_repeats == 3 {
                    return None;
                }
                new_repeats += 1;
            } else {
                new_repeats = 1;
            }
            let path_append = path_nodes.push(pos);
            return Some(Reverse(HeapEntry(
                path_cost + manhattan(pos, dest_pos),
                path_cost + *grid.get(&pos) as u32,
                dir,
                new_repeats,
                path_append,
            )));
        });
        candidate_paths.extend(nexts);
    }
}

fn neighbor_steps(pos: Coord) -> impl Iterator<Item = Cursor> {
    [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .map(|d| (pos + d, d))
    .into_iter()
}

fn manhattan(c1: Coord, c2: Coord) -> u32 {
    (c1.0 - c2.0).abs() as u32 + (c1.1 - c2.1).abs() as u32
}

fn parse_grid(lines: impl Iterator<Item = impl AsRef<str>>) -> Grid<u8> {
    let mut grid = Grid::new();
    for line in lines {
        let row = line
            .as_ref()
            .chars()
            .map(|c| -> u8 { c.to_string().parse().unwrap() })
            .collect();
        grid.rows.push(row)
    }
    grid
}

//fn get_initial_cursors() -> impl Iterator<Item = Cursor> {
//    [((0, 1), Direction::Right), ((1, 0), Direction::Down)].into_iter()
//}
