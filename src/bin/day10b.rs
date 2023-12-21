use advent_of_code_2023::{cursor_grid::*, day10::*};
use std::{
    collections::BTreeSet,
    io::{self, BufRead},
};

type Num = u32;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.expect("IO Error"));
    let result = day10b(lines);
    println!("{}", result);
}

fn day10b(lines: impl Iterator<Item = impl AsRef<str>>) -> Num {
    let (grid, start_pos) = parse_grid(lines);
    let pipe_loop_iter = PipeLoopIterator::new(start_pos, &grid);
    let mut boundary_grid = make_boundary_grid(grid.rows[0].len(), grid.rows.len());
    // Draw boundary on boundary grid
    for (pos, pipe) in pipe_loop_iter {
        // Write pipe boundary data into boundary_grid.
        // boundary_grid is 3x the size of grid (3x3 bits per tile)
        let current_cell_center = (pos.0 * 3 + 1, pos.1 * 3 + 1);
        // paint center
        boundary_grid.set(current_cell_center, InOutBoundary::Boundary);
        // paint connections
        for dir in pipe.connections.iter() {
            boundary_grid.set(current_cell_center + *dir, InOutBoundary::Boundary);
        }
    }
    // Fill outside from top-left corner
    paint_fill(&mut boundary_grid, (0, 0), InOutBoundary::Outside);
    // Count Inside
    boundary_grid
        .rows
        .iter()
        // Reduce 3x3 cells to 1x1 by taking only their centers
        // center rows
        .skip(1)
        .step_by(3)
        // center cols of each center row
        .flat_map(|row| row.iter().skip(1).step_by(3))
        .filter(|state| **state == InOutBoundary::Inside)
        .count()
        .try_into()
        .unwrap()
}

#[derive(Clone, Eq, PartialEq)]
enum InOutBoundary {
    Inside,
    Outside,
    Boundary,
}

fn make_boundary_grid(width: usize, height: usize) -> Grid<InOutBoundary> {
    let mut grid = Grid::new();
    for _ in 0..3 * height {
        grid.rows.push(vec![InOutBoundary::Inside; 3 * width]);
    }
    return grid;
}

fn paint_fill<N>(grid: &mut Grid<N>, pos: Coord, new_color: N)
where
    N: Clone + Eq,
{
    let old_color = grid.get(pos).clone();
    if new_color == old_color {
        // Optimization: already painted; nothing to do.
        return;
    }
    let mut frontier: BTreeSet<Coord> = BTreeSet::new();
    frontier.insert(pos);

    while let Some(current_pos) = frontier.pop_first() {
        grid.set(current_pos, new_color.clone());
        let neighbors_to_paint = grid
            .neighbors(current_pos)
            .filter(|neighbor_pos| *grid.get(*neighbor_pos) == old_color);
        frontier.extend(neighbors_to_paint);
    }
}
