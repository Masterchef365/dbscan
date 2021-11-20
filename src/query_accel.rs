use std::collections::HashMap;
use crate::{Point, IntPoint, distance};

/// Euclidean neighborhood query accelerator. Uses a hashmap grid
pub struct QueryAccelerator {
    cells: HashMap<IntPoint, Vec<usize>>,
    radius: f32,
}

impl QueryAccelerator {
    pub fn new(points: &[Point], radius: f32) -> Self {
        let mut cells: HashMap<IntPoint, Vec<usize>> = HashMap::new();
        for (idx, &point) in points.iter().enumerate() {
            cells
                .entry(Self::calc_cell(point, radius))
                .or_default()
                .push(idx);
        }

        Self { cells, radius }
    }

    /// This should result in better cache locality for queries, but may take some time.
    pub fn sort_indices(mut self) -> Self {
        for indices in self.cells.values_mut() {
            indices.sort();
        }
        self
    }

    // Query the neighbors of `queried_idx` in `points`
    pub fn query_neighbors<'s>(
        &'s self,
        points: &'s [Point],
        queried_idx: usize,
    ) -> impl Iterator<Item = usize> + 's {
        let query_point = points[queried_idx];
        let [origin_x, origin_y] = Self::calc_cell(query_point, self.radius);

        const NEIGHBOR_CELLS: [IntPoint; 9] = [
            [-1, -1],
            [0, -1],
            [1, -1],
            [-1, 0],
            [0, 0],
            [1, 0],
            [-1, 1],
            [0, 1],
            [1, 1],
        ];

        NEIGHBOR_CELLS
            .into_iter()
            .map(move |[dx, dy]| {
                let key = [origin_x + dx, origin_y + dy];
                self.cells.get(&key).map(|cell_indices| {
                    cell_indices.iter().copied().filter(move |&idx| {
                        idx != queried_idx && distance(points[idx], query_point) <= self.radius
                    })
                })
            })
            .flatten()
            .flatten()
    }

    fn calc_cell([x, y]: Point, radius: f32) -> IntPoint {
        let calc = |v: f32| (v / radius).floor() as i32;
        [calc(x), calc(y)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answers_agree() {
        let n_points = 1000;
        let n_test_indices = 100;
        let scale = 3.343;

        let radius = 0.75;

        let points = random_points(n_points, scale);

        let query_accel = QueryAccelerator::new(&points, radius);

        for &test_idx in &random_indices(n_test_indices, points.len()) {
            let mut naive: Vec<usize> = naive_query(&points, test_idx, radius).collect();
            let mut accel: Vec<usize> = query_accel.query_neighbors(&points, test_idx).collect();
            naive.sort();
            accel.sort();
            assert_eq!(naive, accel);
        }
    }
}