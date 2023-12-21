use std::ops::Add;

pub struct Grid<Node> {
    pub rows: Vec<Vec<Node>>,
}
impl<Node> Grid<Node> {
    pub fn new() -> Self {
        Self { rows: Vec::new() }
    }

    pub fn get(self: &Self, coord: Coord) -> &Node {
        assert!(self.is_within_bounds(coord));
        &self.rows[coord.0 as usize][coord.1 as usize]
    }

    pub fn is_within_bounds(&self, coord: Coord) -> bool {
        (0..self.rows.len() as i32).contains(&coord.0)
            && (0..self.rows[0].len() as i32).contains(&coord.1)
    }
}
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn flipped(self) -> Self {
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
pub type Coord = (i32, i32);
pub type Cursor = (Coord, Direction);
