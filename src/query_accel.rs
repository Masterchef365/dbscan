use crate::distance_sq;
use std::collections::HashMap;

/// Euclidean neighborhood query accelerator. Uses a hashmap grid
pub struct QueryAccelerator<const D: usize> {
    cells: HashMap<[i32; D], Vec<usize>>,
    neighbors: Vec<[i32; D]>,
    radius: f32,
    radius_sq: f32,
}

impl<const D: usize> QueryAccelerator<D> {
    pub fn new(points: &[[f32; D]], radius: f32) -> Self {
        let mut cells: HashMap<[i32; D], Vec<usize>> = HashMap::new();

        for (idx, &point) in points.iter().enumerate() {
            cells.entry(quantize(point, radius)).or_default().push(idx);
        }

        let neighbors = neighborhood::<D>();

        Self {
            cells,
            radius,
            radius_sq: radius * radius,
            neighbors,
        }
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
        points: &'s [[f32; D]],
        queried_idx: usize,
    ) -> impl Iterator<Item = usize> + 's {
        let query_point = points[queried_idx];
        let origin = quantize(query_point, self.radius);

        self.neighbors
            .iter()
            .map(move |diff| {
                let key = add(origin, *diff);
                self.cells.get(&key).map(|cell_indices| {
                    cell_indices.iter().copied().filter(move |&idx| {
                        idx != queried_idx
                            && distance_sq(points[idx], query_point) <= self.radius_sq
                    })
                })
            })
            .flatten()
            .flatten()
    }
}

fn add<const D: usize>(mut a: [i32; D], b: [i32; D]) -> [i32; D] {
    a.iter_mut().zip(b).for_each(|(a, b)| *a += b);
    a
}

fn quantize<const D: usize>(p: [f32; D], radius: f32) -> [i32; D] {
    p.map(|v| (v / radius).floor() as i32)
}

fn neighborhood<const D: usize>() -> Vec<[i32; D]> {
    combos(-1, 1, 1)
}

fn combos<const D: usize>(min: i32, max: i32, step: i32) -> Vec<[i32; D]> {
    let mut dims = [min; D];
    let mut combos = vec![];
    loop {
        combos.push(dims);
        if dims == [max; D] {
            break combos;
        }
        for i in 0..dims.len() {
            if dims[i] < max {
                dims[i] += step;
                break;
            } else {
                dims[i] = min;
            }
        }
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

        let points = crate::random_points::<2>(n_points, scale);

        let query_accel = QueryAccelerator::new(&points, radius);

        for &test_idx in &crate::random_indices(n_test_indices, points.len()) {
            let mut naive: Vec<usize> = crate::naive_query(&points, test_idx, radius).collect();
            let mut accel: Vec<usize> = query_accel.query_neighbors(&points, test_idx).collect();
            naive.sort();
            accel.sort();
            assert_eq!(naive, accel);
        }
    }

    #[test]
    fn test_neighbors() {
        let mut expect = vec![
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
        expect.sort();
        let mut output = neighborhood::<2>();
        output.sort();

        assert_eq!(expect, output);
    }
}
