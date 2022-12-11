use crate::prelude::*;

pub struct DijkstraPartial;

impl PathAlgorithm for DijkstraPartial {
    fn compute_path<T, const DIMENSIONS: UVec2>(
        origin: Position<DIMENSIONS>,
        destination: Position<DIMENSIONS>,
        provider: &mut impl PathProvider<T, DIMENSIONS>,
        mut pass_through_data: T,
    ) -> Vec<Position<DIMENSIONS>> {
        let (paths, _) = dijkstra_partial(
            &origin,
            |&p| {
                let neighbors = provider.get_neighbors(p, &mut pass_through_data);
                let mut successors = Vec::with_capacity(neighbors.len());
                for neighbor in neighbors {
                    successors.push((neighbor, provider.cost(neighbor, &mut pass_through_data)));
                }
                successors
            },
            |&p| p == destination,
        );

        let target = paths
            .iter()
            .min_by(|(a_pt, (_, a_cost)), (b_pt, (_, b_cost))| {
                let a_dist = a_pt.distance(destination);
                let b_dist = b_pt.distance(destination);
                (a_dist + *a_cost).cmp(&(b_dist + *b_cost))
            })
            .map(|(pt, _)| pt)
            .unwrap_or(&origin);

        build_path(target, &paths)
    }
}
