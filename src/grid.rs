use std::collections::BTreeSet;

pub type Point = (usize, usize);

/// Parses the galaxy coordinates while taking universe expansion
/// into account.
pub fn parse_galaxy_coordinates<const EXPANSION_FACTOR: usize>(grid: &str) -> Vec<Point> {
    let mut row_num = 0;

    let mut empty_columns: BTreeSet<usize> =
        { BTreeSet::from_iter(0..grid.lines().next().map(str::len).unwrap_or(0)) };

    let mut galaxy_coordinates = Vec::new();
    for row in grid.lines() {
        let mut contained_galaxy = false;
        for (column, character) in row.as_bytes().iter().copied().enumerate() {
            if character == b'#' {
                empty_columns.remove(&column);
                contained_galaxy = true;
                galaxy_coordinates.push((row_num, column));
            }
        }
        row_num += if contained_galaxy {
            1
        } else {
            EXPANSION_FACTOR
        };
    }

    // We have so far only taken rows into account when expanding the universe. We now also take
    // care of columns.
    for (_, column) in &mut galaxy_coordinates {
        // Find the number of empty columns before our galaxy
        let num_previous_empty_columns = empty_columns
            .iter()
            .take_while(|col_num| *col_num < column)
            .enumerate()
            .last()
            .map(|(num_prev_empty, _)| num_prev_empty + 1)
            .unwrap_or(0);
        *column += num_previous_empty_columns * (EXPANSION_FACTOR - 1);
    }
    galaxy_coordinates
}
