use crate::prelude::*;

pub struct AdamsFov;

impl FovAlgorithm for AdamsFov {
    fn compute_fov<T, const DIM: UVec2>(
        origin: Position<DIM>,
        range: u32,
        provider: &mut impl FovProvider<T, DIM>,
        mut pass_through_data: T,
    ) -> HashSet<Position<DIM>> {
        let mut visible_points: HashSet<Position<_>> =
            HashSet::with_capacity(((range * 2) * (range * 2)) as usize);

        visible_points.insert(origin);

        for octant in 0..8 {
            Self::compute_octant(
                octant,
                origin,
                range as i32,
                1,
                Slope { x: 1, y: 1 },
                Slope { x: 1, y: 0 },
                provider,
                &mut pass_through_data,
                &mut visible_points,
            )
        }

        visible_points
    }
}

impl AdamsFov {
    #[allow(clippy::too_many_arguments)]
    fn compute_octant<T, const DIM: UVec2>(
        octant: i32,
        origin: Position<DIM>,
        range: i32,
        x: i32,
        mut top: Slope,
        mut bottom: Slope,
        provider: &mut impl FovProvider<T, DIM>,
        pass_through_data: &mut T,
        visible_points: &mut HashSet<Position<DIM>>,
    ) {
        for x in x..=range {
            let y_coords = Self::compute_y(
                octant,
                origin,
                x,
                &mut top,
                &mut bottom,
                provider,
                pass_through_data,
            );

            let top_y = y_coords.x;
            let bottom_y = y_coords.y;

            if !Self::compute_visiblity(
                top_y,
                bottom_y,
                range,
                octant,
                origin,
                x,
                &mut top,
                &mut bottom,
                provider,
                pass_through_data,
                visible_points,
            ) {
                break;
            }
        }
    }

    fn compute_y<T, const DIM: UVec2>(
        octant: i32,
        origin: Position<DIM>,
        x: i32,
        top: &mut Slope,
        bottom: &mut Slope,
        provider: &mut impl FovProvider<T, DIM>,
        pass_through_data: &mut T,
    ) -> IVec2 {
        let mut top_y;
        if top.x == 1 {
            top_y = x;
        } else {
            top_y = ((x * 2 - 1) * top.y + top.x) / (top.x * 2);

            if Self::blocks_light(x, top_y, octant, origin, provider, pass_through_data) {
                if top.greater_or_equal(top_y * 2 + 1, x * 2) &&
                    !Self::blocks_light(x, top_y + 1, octant, origin, provider, pass_through_data)
                {
                    top_y += 1;
                }
            } else {
                let mut ax = x * 2;
                if Self::blocks_light(
                    x + 1,
                    top_y + 1,
                    octant,
                    origin,
                    provider,
                    pass_through_data,
                ) {
                    ax += 1;
                }
                if top.greater(top_y * 2 + 1, ax) {
                    top_y += 1;
                }
            }
        }

        let mut bottom_y;
        if bottom.y == 0 {
            bottom_y = 0;
        } else {
            bottom_y = ((x * 2 - 1) * bottom.y + bottom.x) / (bottom.x * 2);

            if bottom.greater_or_equal(bottom_y * 2 + 1, x * 2) &&
                Self::blocks_light(x, bottom_y, octant, origin, provider, pass_through_data) &&
                !Self::blocks_light(x, bottom_y + 1, octant, origin, provider, pass_through_data)
            {
                bottom_y += 1;
            }
        }

        IVec2::new(top_y, bottom_y)
    }

    #[allow(clippy::too_many_arguments)]
    fn compute_visiblity<T, const DIM: UVec2>(
        top_y: i32,
        bottom_y: i32,
        range: i32,
        octant: i32,
        origin: Position<DIM>,
        x: i32,
        top: &mut Slope,
        bottom: &mut Slope,
        provider: &mut impl FovProvider<T, DIM>,
        pass_through_data: &mut T,
        visible_points: &mut HashSet<Position<DIM>>,
    ) -> bool {
        let mut was_opaque = -1;

        for y in (bottom_y..=top_y).rev() {
            if range < 0 ||
                Self::distance_squared(Position::<DIM>::ZERO, IVec2::new(x, y)) <=
                    (range as u64 * range as u64)
            {
                let is_opaque = Self::blocks_light(x, y, octant, origin, provider, pass_through_data);

                // Better symmetry
                let is_visible = is_opaque || // Remove is_opaque check for full symmetry but more artifacts in hallways
                (
                    (y != top_y || top.greater_or_equal(y, x)) &&
                    (y != bottom_y || bottom.less_or_equal(y, x))
                );

                if is_visible {
                    Self::set_visible(x, y, octant, origin, visible_points);
                }

                if x != range {
                    if is_opaque {
                        if was_opaque == 0 {
                            let mut nx = x * 2;
                            let ny = y * 2 + 1;
                            if Self::blocks_light(x, y + 1, octant, origin, provider, pass_through_data) {
                                nx -= 1;
                            }
                            if top.greater(ny, nx) {
                                if y == bottom_y {
                                    *bottom = Slope { y: ny, x: nx };
                                    break;
                                } else {
                                    Self::compute_octant(
                                        octant,
                                        origin,
                                        range,
                                        x + 1,
                                        *top,
                                        Slope { y: ny, x: nx },
                                        provider,
                                        pass_through_data,
                                        visible_points,
                                    );
                                }
                            } else if y == bottom_y {
                                return false;
                            }
                        }
                        was_opaque = 1;
                    } else {
                        if was_opaque > 0 {
                            let mut nx = x * 2;
                            let ny = y * 2 + 1;
                            if Self::blocks_light(x + 1, y + 1, octant, origin, provider, pass_through_data) {
                                nx += 1;
                            }
                            if bottom.greater_or_equal(ny, nx) {
                                return false;
                            }
                            *top = Slope { y: ny, x: nx };
                        }
                        was_opaque = 0;
                    }
                }
            }
        }

        was_opaque == 0
    }

    pub fn distance_squared<const DIM: UVec2>(origin: Position<DIM>, tile: IVec2) -> u64 {
        // we don't care about position, so no need to transform the tile
        let end = origin + tile;
        let dx = end.absolute_x() - origin.absolute_x();
        let dy = end.absolute_y() - origin.absolute_y();

        // multiplying times itself is always positive
        (dx * dx + dy * dy) as u64
    }

    fn blocks_light<T, const DIM: UVec2>(
        x: i32,
        y: i32,
        octant: i32,
        mut origin: Position<DIM>,
        provider: &mut impl FovProvider<T, DIM>,
        pass_through_data: &mut T,
    ) -> bool {
        origin.set_xy(Self::transform(x, y, octant, origin));
        provider.is_opaque(origin, pass_through_data)
    }

    fn set_visible<const DIM: UVec2>(
        x: i32,
        y: i32,
        octant: i32,
        mut origin: Position<DIM>,
        visible_points: &mut HashSet<Position<DIM>>,
    ) {
        origin.set_xy(Self::transform(x, y, octant, origin));
        visible_points.insert(origin);
    }

    fn transform<const DIM: UVec2>(x: i32, y: i32, octant: i32, origin: Position<DIM>) -> UVec2 {
        let (mut nx, mut ny): (i32, i32) = origin.gridpoint().as_ivec2().into();
        match octant {
            0 => {
                nx += x;
                ny -= y;
            },
            1 => {
                nx += y;
                ny -= x;
            },
            2 => {
                nx -= y;
                ny -= x;
            },
            3 => {
                nx -= x;
                ny -= y;
            },
            4 => {
                nx -= x;
                ny += y;
            },
            5 => {
                nx -= y;
                ny += x;
            },
            6 => {
                nx += y;
                ny += x;
            },
            7 => {
                nx += x;
                ny += y;
            },
            _ => {},
        }

        IVec2::new(nx, ny).as_uvec2()
    }
}
