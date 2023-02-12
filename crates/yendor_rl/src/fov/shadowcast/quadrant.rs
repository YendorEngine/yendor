use crate::prelude::*;

pub struct Quadrant<'a, T> {
    origin: IVec2,
    direction: Direction,
    pass_through_data: &'a mut T,
    provider: &'a mut dyn FovProvider<T>,
}

impl<'a, T> Quadrant<'a, T> {
    pub fn new(
        direction: Direction,
        origin: IVec2,
        provider: &'a mut dyn FovProvider<T>,
        pass_through_data: &'a mut T,
    ) -> Self {
        Self {
            direction,
            origin,
            provider,
            pass_through_data,
        }
    }

    // adjust the transform based on which direction we are scanning
    const fn transform(&self, tile: IVec2) -> IVec2 {
        if self.direction.has_south() {
            IVec2::new(tile.y, -tile.x)
        } else if self.direction.has_north() {
            IVec2::new(tile.y, tile.x)
        } else if self.direction.has_east() {
            IVec2::new(tile.x, tile.y)
        } else {
            IVec2::new(-tile.x, tile.y)
        }
    }

    pub const fn distance_squared(&self, tile: IVec2) -> u64 {
        (tile.x * tile.x + tile.y * tile.y) as u64
    }

    // mark this tile as visible
    pub fn set_visible(&mut self, visible_points: &mut HashSet<IVec2>, tile: IVec2) {
        visible_points.insert(self.origin + self.transform(tile));
    }

    // check if this tile is opaque
    pub fn is_opaque(&mut self, tile: IVec2) -> bool {
        self.provider
            .is_opaque(self.origin + self.transform(tile), self.pass_through_data)
    }

    pub fn is_clear(&mut self, tile: IVec2) -> bool {
        !self.is_opaque(tile)
    }
}
