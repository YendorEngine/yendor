use crate::prelude::*;

pub struct DijkstraPartial;

impl PathAlgorithm for DijkstraPartial {
    fn compute_path<T>(
        origin: ChunkPosition,
        destination: ChunkPosition,
        provider: &mut impl PathProvider<T>,
        mut pass_through_data: T,
    ) -> Vec<ChunkPosition> {
        let (paths, _) = dijkstra_partial(
            &origin,
            |&p| provider.generate_successors(p, &mut pass_through_data),
            |&p| p == destination,
        );

        let target = paths
            .iter()
            .min_by(|(a_pt, (_, a_cost)), (b_pt, (_, b_cost))| {
                let a_dist = a_pt.distance(destination).unwrap();
                let b_dist = b_pt.distance(destination).unwrap();
                (a_dist + *a_cost).cmp(&(b_dist + *b_cost))
            })
            .map(|(pt, _)| pt)
            .unwrap_or(&origin);

        build_path(target, &paths)
    }
}
