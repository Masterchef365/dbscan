use rand::distributions::{Distribution, Uniform};
mod query_accel;
pub use query_accel::QueryAccelerator;

pub type Point = [f32; 2];
pub type IntPoint = [i32; 2];

/// Euclidean distance between two points
pub fn distance([x1, y1]: Point, [x2, y2]: Point) -> f32 {
    ((x1 - x2).powf(2.) + (y1 - y2).powf(2.)).sqrt()
}

/// Execute an O(n) query to find the indices of all points within `radius` of `queried_idx` in `points`
pub fn naive_query<'points>(
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

/// Create n random points in the range (-size, size) for X and Y
pub fn random_points(n: usize, size: f32) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let dist = Uniform::new(-size, size);
    (0..n)
        .map(|_| [dist.sample(&mut rng), dist.sample(&mut rng)])
        .collect()
}

/// Create n random indices in the range 0..max
pub fn random_indices(n: usize, max: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let distr = Uniform::new(0, max);
    distr.sample_iter(&mut rng).take(n).collect()
}