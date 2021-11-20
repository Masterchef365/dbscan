use std::{collections::HashMap, time::Instant};
use rand::distributions::{Distribution, Uniform};

type Point = [f32; 2];
type IntPoint = [i32; 2];

fn random_points(n: usize, size: f32) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let dist = Uniform::new(-size, size);
    (0..n).map(|_| 
        [ 
            dist.sample(&mut rng),
            dist.sample(&mut rng)
        ]
    ).collect()
}

fn random_indices(n: usize, max: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let distr = Uniform::new(0, max);
    distr.sample_iter(&mut rng).take(n).collect()
}

fn distance([x1, y1]: Point, [x2, y2]: Point) -> f32 {
    ((x1 - x2).powf(2.) + (y1 - y2).powf(2.)).sqrt()
}

fn naive_query(points: &[Point], queried_idx: usize, radius: f32) -> Vec<usize> {
    let queried_point = points[queried_idx];
    let mut indices = vec![];
    for (point_idx, point) in points.iter().enumerate() {
        if point_idx == queried_idx {
            continue;
        }

        if distance(queried_point, *point) <= radius {
            indices.push(point_idx);
        }
    }
    indices
}

struct QueryAccelerator {
    cells: HashMap<IntPoint, Vec<usize>>,
    radius: f32,
}

impl QueryAccelerator {
    fn new(points: &[Point], radius: f32) -> Self {
        let mut cells: HashMap<IntPoint, Vec<usize>> = HashMap::new();
        for (idx, &point) in points.iter().enumerate() {
            cells
                .entry(Self::calc_cell(point, radius))
                .or_default()
                .push(idx);
        }
        
        //TODO: See if this speeds up queries do to cache locality!
        for indices in cells.values_mut() {
            indices.sort();
        }

        Self {
            cells,
            radius,
        }
    }

    fn query_neighbors(&self, points: &[Point], queried_idx: usize) -> Vec<usize> {
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

        let mut indices = vec![];

        for [dx, dy] in NEIGHBOR_CELLS {
            let key = [origin_x + dx, origin_y + dy];
            if let Some(cell_indices) = self.cells.get(&key) {
                for &idx in cell_indices {
                    if idx != queried_idx {
                        if distance(points[idx], query_point) <= self.radius {
                            indices.push(idx);
                        }
                    }
                }
            }
        }

        indices
    }

    fn calc_cell([x, y]: Point, radius: f32) -> IntPoint {
        let calc = |v: f32| (v / radius).floor() as i32;
        [calc(x), calc(y)]
    }
}

fn main() {
    let radius = 0.75;
    let points = random_points(10_000, 1.);
    let query_accel = QueryAccelerator::new(&points, radius);

    let random_indices = random_indices(100_000, points.len());
    let start = Instant::now();
    for &idx in &random_indices {
        let indices = naive_query(&points, idx, radius);
    }
    println!("Naive took {}s", start.elapsed().as_secs_f32());

    let start = Instant::now();
    let accel = QueryAccelerator::new(&points, radius);
    for &idx in &random_indices {
        let indices = accel.query_neighbors(&points, idx);
    }
    println!("Accelerator took {}s", start.elapsed().as_secs_f32());

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
            let mut naive = naive_query(&points, test_idx, radius);
            let mut accel = query_accel.query_neighbors(&points, test_idx);
            naive.sort();
            accel.sort();
            assert_eq!(naive, accel);
        }
    }
}