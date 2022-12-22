use crate::prelude::*;

pub struct IDAstar;

impl PathAlgorithm for IDAstar {
    fn compute_path<T>(
        origin: UVec2,
        destination: UVec2,
        provider: &mut impl PathProvider<T>,
        mut pass_through_data: T,
    ) -> Vec<UVec2> {
        let result = idastar(
            &origin,
            |&p| provider.generate_successors(p, &mut pass_through_data),
            |&p| provider.distance(p, destination),
            |&p| p == destination,
        );

        match result {
            Some((path, _length)) => path,
            None => Vec::new(),
        }
    }
}
