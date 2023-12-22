use advent_of_code_2023::cursor_grid::*;
use advent_of_code_2023::linked_list::LinkedList;
use std::{
    cmp::{max, min, Reverse},
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
    let dest_pos = (grid.rows.len() as i32 - 1, grid.rows[0].len() as i32 - 1);
    assert!(&grid.is_within_bounds(dest_pos));
    let mut candidate_paths: BinaryHeap<Reverse<HeapEntry>> = [Reverse(HeapEntry(
        manhattan(start_pos, dest_pos),    // heuristic
        0,                                 // path_cost
        Direction::Right,                  // current_direction
        0,                                 // direction_repeats
        [start_pos].into_iter().collect(), // path_nodes
    ))]
    .into_iter()
    .collect();
    //let mut visited: HashSet<(Coord, Direction, u8)> = HashSet::new();
    loop {
        let Reverse(HeapEntry(
            heuristic_val,
            path_cost,
            current_direction,
            direction_repeats,
            path_nodes,
        )) = candidate_paths.pop().unwrap();
        let current_pos = path_nodes.head().unwrap();
        println!(
            "{} {} ({}, {})",
            heuristic_val, path_cost, current_pos.0, current_pos.1
        );
        if *current_pos == dest_pos {
            return path_cost;
        }
        let nexts = neighbor_steps(*current_pos).filter_map(|(pos, dir)| {
            if !grid.is_within_bounds(pos) {
                return None;
            }
            //if path_nodes.contains(&pos) {
            //    return None;
            //}
            if dir == current_direction.flipped() {
                // No U-turns
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
            let new_cost = path_cost + *grid.get(pos) as u32;
            return Some(Reverse(HeapEntry(
                new_cost + heuristic(pos, dest_pos, &grid),
                new_cost,
                dir,
                new_repeats,
                path_append,
            )));
        });

        // let nexts_tmp: Vec<_> = nexts
        //     .filter(|Reverse((_, _, dir, dirnim, ll))| {
        //         !visited.contains((dir, dirnim, ll.head().uwrap()))
        //     })
        //     .collect();
        // visited.extend(
        //     nexts_tmp
        //         .iter()
        //         .map(|Reverse((_, _, dir, dirnim, ll))| (dir, dirnim, ll.head())),
        // );

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

fn heuristic(pos: Coord, dest: Coord, grid: &Grid<u8>) -> u32 {
    let mut sum = 0;
    // min of each row between pos and dest inclusive
    // FIXME only works if start above dest
    let row_first: usize = pos.0 as usize + 1;
    let row_last: usize = dest.0 as usize;
    let row_range = row_first..=row_last;
    if !row_first > row_last {
        sum += grid
            .rows
            .iter()
            .skip(row_first as usize)
            .take((row_last + 1 - row_first) as usize)
            .map(|r| *r.iter().reduce(min).unwrap() as u32)
            .sum::<u32>();
    }
    // min of each column between pos and dest inclusive
    // FIXME only works if start left of dest
    let col_first: usize = pos.1 as usize + 1;
    let col_last: usize = dest.1 as usize;
    let col_range = col_first..=col_last;
    if !col_first > col_last {
        let mut min_each_col = vec![9; col_range.clone().count()]; //Vec::with_capacity(col_last - col_first + 1);
        for col_idx in col_range {
            for row_idx in row_range.clone() {
                //if col_idx == col_first && row_idx == row_first {
                //    continue;
                //}
                let item = grid.rows[row_idx][col_idx];
                min_each_col[(col_idx - col_first) as usize] =
                    min(item, min_each_col[(col_idx - col_first) as usize]);
            }
        }
        sum += min_each_col.iter().map(|n| *n as u32).sum::<u32>();
    }
    sum
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
