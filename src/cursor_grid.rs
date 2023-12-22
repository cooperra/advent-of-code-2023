use std::{collections::BTreeSet, ops::Add};

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

    pub fn set(&mut self, coord: Coord, item: Node) {
        assert!(self.is_within_bounds(coord));
        self.rows[coord.0 as usize][coord.1 as usize] = item;
    }

    pub fn iter<'a>(&'a self) -> std::iter::Flatten<std::slice::Iter<'a, Vec<Node>>> {
        let result = self.rows.iter().flatten();
        result
    }

    pub fn valid_neighbor_cursors<'a>(&'a self, pos: Coord) -> impl Iterator<Item = Cursor> + 'a {
        neighbor_cursors(pos).filter(|(pos, _)| self.is_within_bounds(*pos))
    }

    pub fn neighbors(&self, pos: Coord) -> impl Iterator<Item = Coord> + '_ {
        self.valid_neighbor_cursors(pos)
            .map(|(neighbor_pos, _)| neighbor_pos)
    }

    pub fn paint_fill(&mut self, pos: Coord, new_color: Node)
    where
        Node: Clone + Eq,
    {
        let old_color = self.get(pos).clone();
        if new_color == old_color {
            // Optimization: already painted; nothing to do.
            return;
        }
        let mut frontier: BTreeSet<Coord> = BTreeSet::new();
        frontier.insert(pos);

        while let Some(current_pos) = frontier.pop_first() {
            self.set(current_pos, new_color.clone());
            let neighbors_to_paint = self
                .neighbors(current_pos)
                .filter(|neighbor_pos| *self.get(*neighbor_pos) == old_color);
            frontier.extend(neighbors_to_paint);
        }
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

pub fn neighbor_cursors(pos: Coord) -> impl Iterator<Item = Cursor> {
    [Up, Down, Left, Right]
        .into_iter()
        .map(move |dir| ((pos + dir), dir))
}

pub fn manhattan(c1: Coord, c2: Coord) -> u32 {
    (c1.0 - c2.0).abs() as u32 + (c1.1 - c2.1).abs() as u32
}
