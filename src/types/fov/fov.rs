use crate::prelude::*;

pub enum Fov {
    Shadowcast,
    ShadowcastDirection(Direction),
}

impl Fov {
    pub fn compute<FovRange: Into<u32>, T, const GRID_WIDTH: u32, const GRID_HEIGHT: u32>(
        &self,
        origin: Position<GRID_WIDTH, GRID_HEIGHT>,
        range: FovRange,
        provider: &mut impl FovProvider<T, GRID_WIDTH, GRID_HEIGHT>,
        pass_through_data: T,
    ) -> HashSet<Position<GRID_WIDTH, GRID_HEIGHT>> {
        let range = range.into();
        match self {
            Self::Shadowcast => {
                Shadowcast::compute_fov(origin, range, provider, pass_through_data)
            },
            Self::ShadowcastDirection(direction) => {
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
        let visible_sets = Fov::Shadowcast.compute(pos, 1_u32, &mut Provider, data);
        visible_sets.iter().for_each(|pos| {
            println!("{}", pos);
        });
    }
}
