use crate::prelude::*;

pub struct DijkstraPartial;

impl PathAlgorithm for DijkstraPartial {
    fn compute_path<T, const GRID_WIDTH: u32, const GRID_HEIGHT: u32>(
        origin: Position<GRID_WIDTH, GRID_HEIGHT>,
        destination: Position<GRID_WIDTH, GRID_HEIGHT>,
        provider: &mut impl PathProvider<T, GRID_WIDTH, GRID_HEIGHT>,
        mut pass_through_data: T,
    ) -> Vec<Position<GRID_WIDTH, GRID_HEIGHT>> {
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
