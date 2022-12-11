use crate::prelude::*;

// TODO: Add more Fov Algorithms: http://www.adammil.net/blog/v125_Roguelike_Vision_Algorithms.html
// TODO: Adam
// TODO: Ray casting
// TODO: Diamond walls (point-to-tile or point-to-point)
// TODO: Half-width walls (point-to-tile or point-to-point)
// TODO: Permissive field of view (tile-to-tile)
// TODO: Digital field of view (diamond-to-diamond)
pub enum Fov {
    Shadowcast,
    ShadowcastDirection(Direction),
}

impl Fov {
    pub fn compute<FovRange: Into<u32>, T, const DIMENSIONS: UVec2>(
        &self,
        origin: Position<DIMENSIONS>,
        range: FovRange,
        provider: &mut impl FovProvider<T, DIMENSIONS>,
        pass_through_data: T,
    ) -> HashSet<Position<DIMENSIONS>> {
        let range = range.into();
        match self {
            Self::Shadowcast => Shadowcast::compute_fov(origin, range, provider, pass_through_data),
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
        fn is_opaque(&mut self, _position: Position<10, 10>, _pass_through_data: &()) -> bool { false }
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
