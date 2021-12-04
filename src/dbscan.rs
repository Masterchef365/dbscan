use crate::QueryAccelerator;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Label {
    Undefined,
    Noise,
    Cluster(u32),
}

fn dbscan<const D: usize>(points: &[[f32; D]], radius: f32, min_pts: usize) -> Vec<Label> {
    let mut label = vec![Label::Undefined; points.len()];
    let accel = QueryAccelerator::new(points, radius);

    let mut current_cluster = 0;

    for point_idx in 0..points.len() {
        if label[point_idx] != Label::Undefined {
            continue;
        }

        let neighbors = accel.query_neighbors(points, point_idx).collect::<HashSet<usize>>();
        if neighbors.len() < min_pts {
            label[point_idx] = Label::Noise;
            continue;
        }

        current_cluster += 1;
    }

    label
}
