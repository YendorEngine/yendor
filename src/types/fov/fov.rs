use crate::prelude::*;

pub enum Fov<T> {
    Shadowcast(T),
    ShadowcastDirection(Direction, T),
}

impl<T> Fov<T> {
    pub fn compute<FovRange: Into<u32>, const GRID_WIDTH: u32, const GRID_HEIGHT: u32>(
        &self,
        origin: Position<GRID_WIDTH, GRID_HEIGHT>,
        range: FovRange,
        provider: &mut impl FovProvider<T, GRID_WIDTH, GRID_HEIGHT>,
    ) -> HashSet<Position<GRID_WIDTH, GRID_HEIGHT>> {
        let range = range.into();
        match self {
            Self::Shadowcast(pass_through_data) => {
                Shadowcast::compute_fov(origin, range, provider, pass_through_data)
            },
            Self::ShadowcastDirection(direction, pass_through_data) => {
                Shadowcast::compute_direction(origin, range, provider, *direction, pass_through_data)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    struct Provider;
    impl FovProvider<(), 10, 10> for Provider {
        fn is_opaque(&mut self, position: Position<10, 10>, pass_through_data: &()) -> bool { false }
    }

    #[test]
    fn fov_test() {
        let pos: Position<10, 10> = Position::default();
        let data = ();
        let visible_sets = Fov::Shadowcast(data).compute(pos, 1_u32, &mut Provider);
        visible_sets.iter().for_each(|pos| {
            println!("{}", pos);
        });
    }
}
