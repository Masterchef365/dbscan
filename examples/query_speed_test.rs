use neighbor_query::*;
use std::time::Instant;

fn main() {
    let radius = 0.10;
    let points = random_points::<2>(100_000, 1.);

    let random_indices = random_indices(10_000, points.len());

    let start = Instant::now();
    let sum = random_indices
        .iter()
        .copied()
        .map(|idx| naive_query(&points, idx, radius).sum::<usize>())
        .sum::<usize>();
    println!(
        "Naive took {}s, sum = {}",
        start.elapsed().as_secs_f32(),
        sum
    );

    let start = Instant::now();
    let accel = QueryAccelerator::new(&points, radius);
    let sum = random_indices
        .iter()
        .copied()
        .map(|idx| accel.query_neighbors(&points, idx).sum::<usize>())
        .sum::<usize>();
    println!(
        "Accelerator  took {}s, sum = {}",
        start.elapsed().as_secs_f32(),
        sum
    );
}
