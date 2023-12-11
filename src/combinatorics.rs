use crate::coordinates::{Pair, Point};

/// Samples unique pairs of points from the given slice
pub fn pairs(points: &[Point]) -> impl Iterator<Item = Pair> + '_ {
    PairsIter {
        first_idx: 0,
        second_idx: 1,
        points,
    }
}

struct PairsIter<'points> {
    first_idx: usize,
    second_idx: usize,
    points: &'points [Point],
}

// There is possibly a more efficient implementation.
// I didn't bother thinking too much about it as
// this should be much easier to write once we get
// generators on stable.
impl<'points> Iterator for PairsIter<'points> {
    type Item = Pair;
    fn next(&mut self) -> Option<Self::Item> {
        if self.second_idx >= self.points.len() {
            self.first_idx += 1;
            self.second_idx = self.first_idx + 1;
        }
        let (Some(first), Some(second)) = (
            self.points.get(self.first_idx),
            self.points.get(self.second_idx),
        ) else {
            return None;
        };
        self.second_idx += 1;
        Some((*first, *second))
    }
}

#[cfg(test)]
mod tests {
    use super::pairs;

    #[test]
    fn it_works() {
        let points = [(1, 5), (3, 4), (2, 8)];
        let pairs: Vec<_> = pairs(&points).collect();
        assert_eq!(
            pairs,
            [((1, 5), (3, 4)), ((1, 5), (2, 8)), ((3, 4), (2, 8))]
        )
    }
}
