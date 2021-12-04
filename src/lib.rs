use rand::distributions::{Distribution, Uniform};
mod query_accel;
pub use query_accel::QueryAccelerator;

/// Euclidean distance between two points
pub fn distance_sq<const D: usize>(a: [f32; D], b: [f32; D]) -> f32 {
    a.into_iter().zip(b).map(|(a, b)| (a - b) * (a - b)).sum::<f32>()
}

/// Execute an O(n) query to find the indices of all points within `radius` of `queried_idx` in `points`
pub fn naive_query<'points, const D: usize>(
    points: &'points [[f32; D]],
    queried_idx: usize,
    radius: f32,
) -> impl Iterator<Item = usize> + 'points {
    let queried_point = points[queried_idx];
    let radius_sq = radius * radius;
    points
        .iter()
        .enumerate()
        .filter_map(move |(point_idx, point)| {
            (point_idx != queried_idx && distance_sq(queried_point, *point) <= radius_sq)
                .then(|| point_idx)
        })
}

/// Create n random points in the range (-size, size) for X and Y
pub fn random_points<const D: usize>(n: usize, size: f32) -> Vec<[f32; D]> {
    let mut rng = rand::thread_rng();
    let dist = Uniform::new(-size, size);
    (0..n)
        .map(|_| [0.0; D].map(|_| dist.sample(&mut rng)))
        .collect()
}

/// Create n random indices in the range 0..max
pub fn random_indices(n: usize, max: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let distr = Uniform::new(0, max);
    distr.sample_iter(&mut rng).take(n).collect()
}
