use itertools::Itertools;

use crate::cursor_grid::manhattan64 as manhattan;

type Num = i64;
type Coord = (Num, Num);
type Universe = Vec<Coord>;

pub fn day11(lines: impl Iterator<Item = impl AsRef<str>>, multiplier: Num) -> Num {
    let mut universe = parse_grid(lines);
    expand_universe(&mut universe, multiplier);
    sum_distances(&universe)
}

fn parse_grid(lines: impl Iterator<Item = impl AsRef<str>>) -> Universe {
    let mut universe = Vec::new();
    for (row_idx, line) in lines.enumerate() {
        let row_galaxies = line
            .as_ref()
            .chars()
            .enumerate()
            .filter_map(|(col_idx, c)| (c == '#').then_some((row_idx as Num, col_idx as Num)));
        universe.extend(row_galaxies);
    }
    universe
}

fn expand_universe(mut universe: &mut Universe, multiplier: Num) {
    replicate_empty_rows(&mut universe, multiplier);
    replicate_empty_columns(&mut universe, multiplier);
}

fn replicate_empty_columns(universe: &mut Universe, multiplier: Num) {
    universe.sort_by_key(|(_, col_idx)| *col_idx);
    let mut col_shift: Num = 0;
    let mut current_col: Num = 0;
    for (_, galaxy_col) in universe.iter_mut() {
        let col_delta: Num = *galaxy_col - current_col;
        if col_delta > 0 {
            current_col = *galaxy_col;
            let empty_col_count = col_delta - 1;
            // When col_delta is 1, this should have no effect, because there are no empty cols.
            let empty_col_count_expanded = empty_col_count * multiplier as Num;
            col_shift += empty_col_count_expanded - empty_col_count;
        }
        *galaxy_col += col_shift;
    }
}

fn replicate_empty_rows(universe: &mut Universe, multiplier: Num) {
    // Assume already sorted by row
    let mut row_shift: Num = 0;
    let mut current_row: Num = 0;
    for (galaxy_row, _) in universe.iter_mut() {
        let row_delta: Num = *galaxy_row - current_row;
        if row_delta > 0 {
            current_row = *galaxy_row;
            let empty_row_count = row_delta - 1;
            // When row_delta is 1, this should have no effect, because there are no empty rows.
            let empty_row_count_expanded = empty_row_count * multiplier as Num;
            row_shift += empty_row_count_expanded - empty_row_count;
        }
        *galaxy_row += row_shift;
    }
}

fn sum_distances(universe: &Universe) -> Num {
    let galaxy_pairs = galaxy_pairs(&universe);
    galaxy_pairs
        .into_iter()
        .map(|pair| manhattan(pair[0], pair[1]))
        .sum()
}

fn galaxy_pairs(universe: &Universe) -> impl Iterator<Item = Vec<Coord>> + Sized + '_ {
    let galaxy_positions = locate_galaxies(universe);
    galaxy_positions.combinations(2)
}

fn locate_galaxies(universe: &Universe) -> impl Iterator<Item = Coord> + '_ {
    universe.clone().into_iter()
}
