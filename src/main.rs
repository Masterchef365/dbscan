use std::collections::HashMap;

use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};

type Point = [f32; 2];
type IntPoint = [i32; 2];

fn random_positions(n: usize, size: f32) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let dist = Uniform::new(-size, size);
    (0..n).map(|_| 
        [ 
            dist.sample(&mut rng),
            dist.sample(&mut rng)
        ]
    ).collect()
}

fn distance([x1, y1]: Point, [x2, y2]: Point) -> f32 {
    ((x1 - x2).powf(2.) + (y1 - y2).powf(2.)).sqrt()
}

fn naive_query(points: &[Point], queried_idx: usize, radius: f32) -> Vec<usize> {
    let queried_point = points[queried_idx];
    let mut indices = vec![];
    for (point_idx, point) in points {
        if idx == queried_point {
            continue;
        }

        if distance(queried_point, point) <= radius {
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
        
        /*
        TODO: See if this speeds up queries do to cache locality!
        for indices in cells.values_mut() {
            indices.sort();
        }
        */

        Self {
            cells,
            radius,
        }
    }

    fn query_neighbors(&self, points: &[Point], queried_idx: usize) -> Vec<usize> {
        let query_point = points[queried_idx];
        let [origin_x, origin_y] = Self::calc_cell(query_point, self.radius);
        const NEIGHBOR_CELLS: [IntPoint; 8] = [
            [-1, -1],
            [0, -1],
            [1, -1],
            [-1, 0],
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
                        if distance(points[idx], query_point) < self.radius {
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
    let positions = random_positions(1000, 1.);
    dbg!(&positions);
}