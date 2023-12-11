use crate::coordinates::Point;

/// The manhattan metric
pub fn manhattan((a, b): Point, (c, d): Point) -> usize {
    a.abs_diff(c) + b.abs_diff(d)
}
