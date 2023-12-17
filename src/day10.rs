use crate::cursor_grid::{Direction::*, *};
use std::{
    collections::HashSet,
    io::{self, BufRead},
};

type Node = Option<Pipe>;
type Row = Vec<Node>;
pub struct Pipe {
    connections: HashSet<Direction>,
}
impl Pipe {
    pub fn from_char(c: &char) -> Option<Self> {
        let connections = match c {
            'J' => [Left, Up],
            'L' => [Up, Right],
            'F' => [Right, Down],
            '7' => [Left, Down],
            '|' => [Up, Down],
            '-' => [Left, Right],
            _ => return None,
        };
        Some(Self {
            connections: HashSet::from(connections),
        })
    }
}

/// get cursors on start pos facing connected adjacent pipes
pub fn get_initial_cursors(start_pos: &Coord, grid: &Grid<Node>) -> Vec<Cursor> {
    let cursors_on_adjacent_connected_pipes = [Up, Down, Left, Right]
        .into_iter()
        .filter_map(|dir| {
            let next_pos = *start_pos + dir;
            let ref maybe_next_pipe = grid.get(&next_pos);
            maybe_next_pipe
                .as_ref()
                .filter(|p| p.connections.contains(&dir.flipped()))
                .and(Some((*start_pos, dir)))
        })
        .collect();
    cursors_on_adjacent_connected_pipes
}

pub fn parse_grid(lines: impl Iterator<Item = impl AsRef<str>>) -> (Grid<Node>, Coord) {
    let mut grid = Grid::new();
    let mut start_pos = None;
    for (row_idx, line) in lines.enumerate() {
        let (row, maybe_start_col) = parse_row(line.as_ref());
        grid.rows.push(row);
        if let Some(col_idx) = maybe_start_col {
            start_pos = Some((row_idx as i32, col_idx as i32));
        }
    }
    (grid, start_pos.expect("No start pos"))
}

pub fn parse_row(line: &str) -> (Row, Option<usize>) {
    let mut row = Row::new();
    let mut maybe_start_col = None;
    for (idx, c) in line.chars().enumerate() {
        let maybe_pipe = Pipe::from_char(&c);
        if maybe_pipe.is_none() {
            if c == 'S' {
                maybe_start_col = Some(idx);
            } else {
                assert_eq!(c, '.');
            }
        }
        row.push(maybe_pipe);
    }
    (row, maybe_start_col)
}

pub trait GridIterator<Node> {
    fn next(self: &Self, grid: &Grid<Node>) -> Self;
}

impl GridIterator<Node> for Cursor {
    /// Move from current pos to next pos, then face pipe's other exit
    fn next(self: &Self, grid: &Grid<Node>) -> Self {
        let (coord, dir) = self;
        let next_coord = *coord + *dir;
        let maybe_next_pipe = grid.get(&next_coord);
        if let Some(next_pipe) = maybe_next_pipe {
            let mut dirset = next_pipe.connections.clone();
            dirset.remove(&dir.flipped());
            assert_eq!(dirset.len(), 1);
            let next_dir = dirset.into_iter().next().unwrap();
            return (next_coord, next_dir);
        } else {
            // This can only happen if we get back to start, or the map is wrong, and that shouldn't happen in this program
            panic!();
        }
    }
}
