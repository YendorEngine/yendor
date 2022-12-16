use crate::prelude::*;

// TODO: Add more Fov Algorithms: http://www.adammil.net/blog/v125_Roguelike_Vision_Algorithms.html
// TODO: Adam
// TODO: Ray casting
// TODO: Diamond walls (point-to-tile or point-to-point)
// TODO: Half-width walls (point-to-tile or point-to-point)
// TODO: Permissive field of view (tile-to-tile)
// TODO: Digital field of view (diamond-to-diamond)
pub enum Fov {
    Adams,
    Shadowcast,
    ShadowcastDirection(Direction),
}

impl Fov {
    pub fn compute<FovRange: Into<u32>, T>(
        &self,
        origin: ChunkPosition,
        range: FovRange,
        provider: &mut impl FovProvider<T>,
        pass_through_data: T,
    ) -> HashSet<ChunkPosition> {
        let range = range.into();
        match self {
            Self::Adams => AdamsFov::compute_fov(origin, range, provider, pass_through_data),
            Self::Shadowcast => Shadowcast::compute_fov(origin, range, provider, pass_through_data),
            Self::ShadowcastDirection(direction) => {
                Shadowcast::compute_direction(origin, range, provider, *direction, pass_through_data)
            },
        }
    }

    pub fn within_fov<FovRange: Into<u32>, T>(
        &self,
        origin: ChunkPosition,
        target: ChunkPosition,
        range: FovRange,
        provider: &mut impl FovProvider<T>,
        pass_through_data: T,
    ) -> bool {
        let range = range.into();
        Self::compute(self, origin, range, provider, pass_through_data).contains(&target)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    struct Provider;
    impl FovProvider<()> for Provider {
        fn is_opaque(&mut self, _position: ChunkPosition, _pass_through_data: &mut ()) -> bool { false }
    }

    mod shadowcast {
        use super::*;

        #[test]
        fn shadowcast() {
            let pos: ChunkPosition = ChunkPosition::new(5, 5, 0);
            let visible_sets = Fov::Shadowcast.compute(pos, 4_u32, &mut Provider, ());
            assert_eq!(visible_sets.len(), 49);

            // Pretty print to canvas for visual inspection
            let mut canvas = Canvas::new(UVec2::new(10, 10));
            visible_sets.iter().for_each(|pos| {
                println!("{}", pos);
                canvas.put(pos.as_uvec2(), '*');
            });
            canvas.print();
        }
    }

    mod adams {
        use super::*;

        #[test]
        fn adams() {
            let pos: ChunkPosition = ChunkPosition::new(5, 5, 0);
            let visible_sets = Fov::Adams.compute(pos, 4_u32, &mut Provider, ());
            assert_eq!(visible_sets.len(), 49);

            // Pretty print to canvas for visual inspection
            let mut canvas = Canvas::new(UVec2::new(10, 10));
            visible_sets.iter().for_each(|pos| {
                println!("{}", pos);
                canvas.put(pos.as_uvec2(), '*');
            });
            canvas.print();
        }
    }
}
