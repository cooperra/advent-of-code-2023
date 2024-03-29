use crate::cursor_grid::{Direction::*, *};
use std::collections::HashSet;

type Node = Option<Pipe>;
type Row = Vec<Node>;
pub struct Pipe {
    pub connections: HashSet<Direction>,
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
pub fn get_initial_cursors(start_pos: Coord, grid: &Grid<Node>) -> Vec<Cursor> {
    let start_pipe = grid.get(start_pos).as_ref().expect("No start pipe");
    start_pipe
        .connections
        .iter()
        .map(|dir| (start_pos, *dir))
        .collect()
}

fn infer_pipe_from_neighbors(pos: Coord, grid: &Grid<Node>) -> Node {
    let directions_to_connected_neighbor_pipes: HashSet<Direction> = grid
        .valid_neighbor_cursors(pos)
        .filter(|(neighbor_pos, dir)| {
            let maybe_neighbor_pipe = grid.get(*neighbor_pos);
            maybe_neighbor_pipe
                .as_ref()
                .filter(|p| p.connections.contains(&dir.flipped()))
                .is_some()
        })
        .map(|(_, dir)| dir)
        .collect();
    if directions_to_connected_neighbor_pipes.len() == 2 {
        Some(Pipe {
            connections: directions_to_connected_neighbor_pipes,
        })
    } else {
        None
    }
}

pub fn parse_grid(lines: impl Iterator<Item = impl AsRef<str>>) -> (Grid<Node>, Coord) {
    let mut grid = Grid::new();
    let mut maybe_start_pos = None;
    for (row_idx, line) in lines.enumerate() {
        let (row, maybe_start_col) = parse_row(line.as_ref());
        grid.rows.push(row);
        if let Some(col_idx) = maybe_start_col {
            maybe_start_pos = Some((row_idx as i32, col_idx as i32));
        }
    }
    let start_pos = maybe_start_pos.expect("No start pos");
    // Start location actually represents a pipe connected to its neighbors.
    // Lets insert it now.
    let start_pipe = infer_pipe_from_neighbors(start_pos, &grid)
        .expect("Start pos isn't connected to exactly 2 pipes");
    grid.rows[start_pos.0 as usize][start_pos.1 as usize] = Some(start_pipe);
    (grid, start_pos)
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

pub struct PipeLoopIterator<'a> {
    start: Coord,
    current_cursor: Option<Cursor>,
    grid: &'a Grid<Node>,
}

impl<'a> PipeLoopIterator<'a> {
    pub fn new(start: Coord, grid: &'a Grid<Node>) -> Self {
        let start_direction = grid
            .get(start)
            .as_ref()
            .map(|pipe| *pipe.connections.iter().next().unwrap());
        let current_cursor = start_direction.map(|dir| (start, dir));
        Self {
            start,
            current_cursor,
            grid,
        }
    }
}

impl<'a> Iterator for PipeLoopIterator<'a> {
    type Item = (Coord, &'a Pipe);
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_cursor.is_none() {
            return None;
        }
        let (pos, _) = self.current_cursor.unwrap();
        let pipe = self.grid.get(pos).as_ref();
        let result = pipe.map(|p| (pos, p));
        let next_cursor = self
            .current_cursor
            .map(|c| c.next(&self.grid))
            // When next is start, we've completed the loop.
            .filter(|c| c.0 != self.start);
        self.current_cursor = next_cursor;
        return result;
    }
}

pub trait GridIterator<Node> {
    fn next(self: &Self, grid: &Grid<Node>) -> Self;
}

impl GridIterator<Node> for Cursor {
    /// Move from current pos to next pos, then face pipe's other exit
    fn next(self: &Self, grid: &Grid<Node>) -> Self {
        let (coord, dir) = self;
        let next_coord = *coord + *dir;
        let maybe_next_pipe = grid.get(next_coord);
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
