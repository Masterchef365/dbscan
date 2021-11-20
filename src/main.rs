use rand::distributions::{Distribution, Uniform};
use std::{collections::HashMap, time::Instant};

type Point = [f32; 2];
type IntPoint = [i32; 2];

fn random_points(n: usize, size: f32) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let dist = Uniform::new(-size, size);
    (0..n)
        .map(|_| [dist.sample(&mut rng), dist.sample(&mut rng)])
        .collect()
}

fn random_indices(n: usize, max: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let distr = Uniform::new(0, max);
    distr.sample_iter(&mut rng).take(n).collect()
}

fn distance([x1, y1]: Point, [x2, y2]: Point) -> f32 {
    ((x1 - x2).powf(2.) + (y1 - y2).powf(2.)).sqrt()
}

fn naive_query<'points>(
    points: &'points [Point],
    queried_idx: usize,
    radius: f32,
) -> impl Iterator<Item = usize> + 'points {
    let queried_point = points[queried_idx];
    points
        .iter()
        .enumerate()
        .filter_map(move |(point_idx, point)| {
            (point_idx != queried_idx && distance(queried_point, *point) <= radius)
                .then(|| point_idx)
        })
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

        Self { cells, radius }
    }

    fn query_neighbors<'s>(
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

fn main() {
    let radius = 0.75;
    let points = random_points(100_000, 1.);

    let random_indices = random_indices(10_000, points.len());
    let start = Instant::now();
    let sum = random_indices.iter().copied().map(|idx| naive_query(&points, idx, radius).sum::<usize>()).sum::<usize>();
    println!("Naive took {}s, sum = {}", start.elapsed().as_secs_f32(), sum);

    let start = Instant::now();
    let accel = QueryAccelerator::new(&points, radius);
    let sum = random_indices.iter().copied().map(|idx| accel.query_neighbors(&points, idx).sum::<usize>()).sum::<usize>();
    println!("Accelerator  took {}s, sum = {}", start.elapsed().as_secs_f32(), sum);
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