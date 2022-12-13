use crate::prelude::*;

pub struct IDAstar;

impl PathAlgorithm for IDAstar {
    fn compute_path<T, const DIM: UVec2>(
        origin: Position<DIM>,
        destination: Position<DIM>,
        provider: &mut impl PathProvider<T>,
        mut pass_through_data: T,
    ) -> Vec<Position<DIM>> {
        let result = idastar(
            &origin,
            |&p| provider.generate_successors(p, &mut pass_through_data),
            |&p| provider.distance::<DIM>(p, destination),
            |&p| p == destination,
        );

        match result {
            Some((path, _length)) => path,
            None => Vec::new(),
        }
    }
}
