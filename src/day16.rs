use crate::cursor_grid::{Direction::*, *};
use std::collections::HashSet;

pub type Num = u32;
pub type Node = Option<MirrorSplitter>;
type Row = Vec<Node>;
#[derive(Debug, Clone, Copy)]
pub enum MirrorType {
    Slash,
    Backslash,
}
#[derive(Debug, Clone, Copy)]
pub enum SplitterType {
    Pipe,
    Dash,
}
#[derive(Debug, Clone, Copy)]
pub enum MirrorSplitter {
    Mirror(MirrorType),
    Splitter(SplitterType),
}
use MirrorSplitter::*;
use MirrorType::*;
use SplitterType::*;

impl MirrorSplitter {
    pub fn from_char(c: &char) -> Option<Self> {
        Some(match c {
            '/' => Mirror(Slash),
            '\\' => Mirror(Backslash),
            '|' => Splitter(Pipe),
            '-' => Splitter(Dash),
            _ => return None,
        })
    }

    pub fn route_light(&self, input: Direction) -> &'static [Direction] {
        match self {
            Mirror(kind) => match input {
                Up => match kind {
                    Slash => &[Right],
                    Backslash => &[Left],
                },
                Down => match kind {
                    Slash => &[Left],
                    Backslash => &[Right],
                },
                Right => match kind {
                    Slash => &[Up],
                    Backslash => &[Down],
                },
                Left => match kind {
                    Slash => &[Down],
                    Backslash => &[Up],
                },
            },
            Splitter(Pipe) => match input {
                Up => &[Up],
                Down => &[Down],
                Left => &[Up, Down],
                Right => &[Up, Down],
            },
            Splitter(Dash) => match input {
                Up => &[Left, Right],
                Down => &[Left, Right],
                Left => &[Left],
                Right => &[Right],
            },
        }
    }
}

pub fn route_light(node: Node, dir: Direction) -> &'static [Direction] {
    let dir_static = match dir {
        Up => &[Up],
        Down => &[Down],
        Left => &[Left],
        Right => &[Right],
    };
    node.map_or(dir_static, |item| item.route_light(dir))
}

pub fn parse_grid(lines: impl Iterator<Item = impl AsRef<str>>) -> Grid<Node> {
    let mut grid = Grid::new();
    for line in lines {
        let row = parse_row(line.as_ref());
        grid.rows.push(row);
    }
    grid
}

pub fn parse_row(line: &str) -> Row {
    let mut row = Row::new();
    for c in line.chars() {
        let maybe_pipe = MirrorSplitter::from_char(&c);
        row.push(maybe_pipe);
    }
    row
}

pub fn count_energized_tiles(start_cursor: Cursor, grid: &Grid<Node>) -> Num {
    let mut cursors = vec![start_cursor];
    let mut energy_map: HashSet<Coord> = HashSet::new();
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
