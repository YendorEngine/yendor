// FoV implementation taken from:
// https://www.albertford.com/shadowcasting/
use super::{quadrant::*, row::*};
use crate::prelude::*;

pub struct Shadowcast;

impl FovAlgorithm for Shadowcast {
    fn compute_fov<T, const DIM: UVec2>(
        origin: Position<DIM>,
        range: u32,
        provider: &mut impl FovProvider<T, DIM>,
        mut pass_through_data: T,
    ) -> HashSet<Position<DIM>> {
        let mut visible_points: HashSet<Position<_>> =
            HashSet::with_capacity(((range * 2) * (range * 2)) as usize);

        visible_points.insert(origin);

        CardinalDirection::all().for_each(|direction| {
            let mut quadrant = Quadrant::new(direction, origin, provider, &mut pass_through_data);
            let mut first_row = Row::new(1, Slope::new(-1, 1), Slope::new(1, 1));
            Self::scan_recursive(range, &mut quadrant, &mut first_row, &mut visible_points);
        });

        visible_points
    }
}

impl Shadowcast {
    pub fn compute_direction<T, const DIM: UVec2>(
        origin: Position<DIM>,
        range: u32,
        provider: &mut impl FovProvider<T, DIM>,
        direction: Direction,
        mut pass_through_data: T,
    ) -> HashSet<Position<DIM>> {
        let mut visible_points: HashSet<Position<_>> =
            HashSet::with_capacity(((range * 2) * (range * 2)) as usize);
        visible_points.insert(origin);

        let mut quadrant = Quadrant::new(direction, origin, provider, &mut pass_through_data);
        let mut first_row = Row::new(1, Slope::new(-1, 1), Slope::new(1, 1));
        Self::scan_recursive(range, &mut quadrant, &mut first_row, &mut visible_points);

        visible_points
    }

    fn scan_recursive<T, const DIM: UVec2>(
        range: u32,
        quadrant: &mut Quadrant<T, DIM>,
        row: &mut Row,
        visible_points: &mut HashSet<Position<DIM>>,
    ) {
        let mut prev_tile = None;
        for tile in row.tiles() {
            if quadrant.distance_squared(tile) > (range as u64 * range as u64) {
                continue;
            }

            // Should we reveal the tile?
            if quadrant.is_opaque(tile) | row.is_symmetric(tile) {
                quadrant.set_visible(visible_points, tile);
            }

            // handle the current row based on vision angles around the previous tile
            if let Some(prev_tile) = prev_tile {
                // did we *just* hit floor after traveling through walls?
                if quadrant.is_opaque(prev_tile) & quadrant.is_clear(tile) {
                    row.calc_starting_slope(tile)
                }
                // did we *just* hit a wall after traveling through floors?
                if quadrant.is_clear(prev_tile) & quadrant.is_opaque(tile) {
                    let mut next_row = row.next();
                    next_row.calc_ending_slope(tile);
                    Self::scan_recursive(range, quadrant, &mut next_row, visible_points);
                }
            }

            // setup for next tile
            prev_tile = Some(tile);
        }

        // if our last tile was floor, we can see down another row
        if let Some(prev_tile) = prev_tile {
            if quadrant.is_clear(prev_tile) {
                Self::scan_recursive(range, quadrant, &mut row.next(), visible_points);
            }
        }
    }
}
