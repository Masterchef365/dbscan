use crate::QueryAccelerator;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Label {
    Undefined,
    Noise,
    Cluster(u32),
}

pub fn dbscan<const D: usize>(points: &[[f32; D]], radius: f32, min_pts: usize) -> Vec<Label> {
    let mut label = vec![Label::Undefined; points.len()];

    let accel = QueryAccelerator::new(points, radius);

    let mut current_cluster = 0;

    // TODO: Don't iterate by point, iterate by query accel chunk (Hope for fewer cache misses)
    for point_idx in 0..points.len() {
        if label[point_idx] != Label::Undefined {
            continue;
        }

        let neighbors = accel
            .query_neighbors(points, point_idx)
            .collect::<Vec<usize>>();

        if neighbors.len() < min_pts {
            label[point_idx] = Label::Noise;
            continue;
        }

        label[point_idx] = Label::Cluster(current_cluster);

        let mut queue = neighbors;

        while let Some(neighbor_idx) = queue.pop() {
            if label[neighbor_idx] == Label::Noise {
                label[neighbor_idx] = Label::Cluster(current_cluster);
            }

            if label[neighbor_idx] != Label::Undefined {
                continue;
            }

            label[neighbor_idx] = Label::Cluster(current_cluster);

            let neighbors = || accel.query_neighbors(points, neighbor_idx);

            if neighbors().count() >= min_pts {
                queue.extend(neighbors());
            }
        }

        current_cluster += 1;
    }

    label
}
