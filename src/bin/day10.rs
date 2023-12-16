use std::{
    collections::HashSet,
    io::{self, BufRead},
    ops::Add,
};

type Num = u32;
struct Grid {
    rows: Vec<Row>,
}
impl Grid {
    fn new() -> Self {
        Grid { rows: Vec::new() }
    }

    fn get(self: &Self, coord: &Coord) -> &Option<Pipe> {
        &self.rows[coord.0][coord.1]
    }
}
type Row = Vec<Option<Pipe>>;
#[derive(Hash, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn flipped(&self) -> Self {
        match self {
            Up => Down,
            Left => Right,
            Down => Up,
            Right => Left,
        }
    }
}

impl Add<Direction> for Coord {
    type Output = Self;
    fn add(self: Self, dir: Direction) -> Self {
        // Fixme: Can over and underflow.
        match dir {
            Up => (self.0 - 1, self.1),
            Left => (self.0, self.1 - 1),
            Down => (self.0 + 1, self.1),
            Right => (self.0, self.1 + 1),
        }
    }
}

use Direction::*;
struct Pipe {
    connections: HashSet<Direction>,
}
type Coord = (usize, usize);
type Cursor = (Coord, Direction);

impl Pipe {
    fn from_char(c: &char) -> Option<Self> {
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

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.expect("IO Error"));
    let result = day16(lines);
    println!("{}", result);
}

fn day16(lines: impl Iterator<Item = impl AsRef<str>>) -> Num {
    let (grid, start_pos) = parse_grid(lines);
    let mut cursors = get_initial_cursors(&start_pos, &grid);
    assert_eq!(2, cursors.len());
    let mut iter_count = 0;
    loop {
        iter_count += 1;
        cursors = cursors
            .into_iter()
            .map(|cursor: Cursor| cursor.next(&grid))
            .collect();
        let ref some_pos = cursors[1].0;
        if cursors.iter().all(|cursor: &Cursor| some_pos == &cursor.0) {
            return iter_count;
        }
    }
}

trait GridIterator {
    fn next(self: &Self, grid: &Grid) -> Self;
}

impl GridIterator for Cursor {
    /// Move from current pos to next pos, then face pipe's other exit
    fn next(self: &Self, grid: &Grid) -> Self {
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

/// get cursors on start pos facing connected adjacent pipes
fn get_initial_cursors(start_pos: &Coord, grid: &Grid) -> Vec<Cursor> {
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

fn parse_grid(lines: impl Iterator<Item = impl AsRef<str>>) -> (Grid, Coord) {
    let mut grid = Grid::new();
    let mut start_pos = None;
    for (row_idx, line) in lines.enumerate() {
        let (row, maybe_start_col) = parse_row(line.as_ref());
        grid.rows.push(row);
        if let Some(col_idx) = maybe_start_col {
            start_pos = Some((row_idx, col_idx));
        }
    }
    (grid, start_pos.expect("No start pos"))
}

fn parse_row(line: &str) -> (Row, Option<usize>) {
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
