use crate::prelude::*;

pub struct Quadrant<'a, T, const DIM: UVec2> {
    direction: Direction,
    pass_through_data: &'a mut T,
    provider: &'a mut dyn FovProvider<T, DIM>,
    origin: Position<DIM>,
}

impl<'a, T, const DIM: UVec2> Quadrant<'a, T, DIM> {
    pub fn new(
        direction: Direction,
        origin: Position<DIM>,
        provider: &'a mut dyn FovProvider<T, DIM>,
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
    fn transform(&self, tile: IVec2) -> IVec2 {
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

    pub fn distance_squared(&self, tile: IVec2) -> u64 {
        // we don't care about position, so no need to transform the tile
        let end = self.origin + tile;
        let dx = end.absolute_x() - self.origin.absolute_x();
        let dy = end.absolute_y() - self.origin.absolute_y();

        // multiplying times itself is always positive
        (dx * dx + dy * dy) as u64
    }

    // mark this tile as visible
    pub fn set_visible(&mut self, visible_points: &mut HashSet<Position<DIM>>, tile: IVec2) {
        visible_points.insert(self.origin + self.transform(tile));
    }

    // check if this tile is opaque
    pub fn is_opaque(&mut self, tile: IVec2) -> bool {
        self.provider.is_opaque(self.origin + self.transform(tile), self.pass_through_data)
    }

    pub fn is_clear(&mut self, tile: IVec2) -> bool { !self.is_opaque(tile) }
}
