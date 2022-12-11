#![allow(dead_code)]

use super::*;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct GridRectangle<const DIMENSIONS: UVec2> {
    pub min: Position<DIMENSIONS>,
    pub max: Position<DIMENSIONS>,
}

impl<const DIMENSIONS: UVec2> GridRectangle<DIMENSIONS> {
    #[inline]
    pub fn new(min: Position<DIMENSIONS>, max: Position<DIMENSIONS>) -> Self {
        let (min, max) = Self::find_min_max(min, max);
        Self { min, max }
    }

    fn find_min_max(
        min: Position<DIMENSIONS>,
        max: Position<DIMENSIONS>,
    ) -> (Position<DIMENSIONS>, Position<DIMENSIONS>) {
        let (abs_min_x, abs_min_y, abs_min_z) = min.absolute_position();
        let (abs_max_x, abs_max_y, abs_max_z) = max.absolute_position();

        let min_x = abs_min_x.min(abs_max_x);
        let max_x = abs_min_x.max(abs_max_x);

        let min_y = abs_min_y.min(abs_max_y);
        let max_y = abs_min_y.max(abs_max_y);

        let min_z = abs_min_z.min(abs_max_z);
        let max_z = abs_min_z.max(abs_max_z);

        (
            Position::from_absolute_position((min_x, min_y, min_z)),
            Position::from_absolute_position((max_x, max_y, max_z)),
        )
    }
}

impl<const DIMENSIONS: UVec2> GridRectangle<DIMENSIONS> {
    #[inline]
    pub const fn width(&self) -> u32 { self.max.x() - self.min.x() }

    #[inline]
    pub const fn height(&self) -> u32 { self.max.y() - self.min.y() }

    #[inline]
    pub const fn min(&self) -> Position<DIMENSIONS> { self.min }

    #[inline]
    pub const fn max(&self) -> Position<DIMENSIONS> { self.max }

    #[inline]
    pub const fn is_square(&self) -> bool { self.width() == self.height() }
}

impl<const DIMENSIONS: UVec2> GridRectangle<DIMENSIONS> {
    #[inline]
    fn center(&self) -> Position<DIMENSIONS> { self.min.lerp(self.max, 0.5) }

    #[inline]
    fn left(&self) -> Position<DIMENSIONS> { self.center() - IVec2::new((self.width() / 2) as i32, 0) }

    #[inline]
    fn right(&self) -> Position<DIMENSIONS> { self.center() + IVec2::new((self.width() / 2) as i32, 0) }

    #[inline]
    fn top(&self) -> Position<DIMENSIONS> { self.center() + IVec2::new(0, (self.height() / 2) as i32) }

    #[inline]
    fn bottom(&self) -> Position<DIMENSIONS> { self.center() - IVec2::new(0, (self.height() / 2) as i32) }

    /// Check if this rectangle intersects another rectangle.
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        let (s_min_x, s_min_y, _s_min_z) = self.min.absolute_position();
        let (s_max_x, s_max_y, _s_max_z) = self.max.absolute_position();

        let (o_min_x, o_min_y, _o_min_z) = other.min.absolute_position();
        let (o_max_x, o_max_y, _o_max_z) = other.max.absolute_position();
        // (self.min.cmple(other.max) & self.max.cmpge(other.min)).all()
        s_min_x <= o_max_x && s_max_x >= o_min_x && s_min_y <= o_max_y && s_max_y >= o_min_y
    }

    /// Calls a function for each x/y point in the rectangle
    pub fn for_each<F>(&self, f: F)
    where F: FnMut(Position<DIMENSIONS>) {
        self.into_iter().for_each(f)
    }
}

impl<const DIMENSIONS: UVec2> Shape<DIMENSIONS> for GridRectangle<DIMENSIONS> {
    #[inline]
    fn get_count(&self) -> u32 { self.width() * self.height() }

    #[inline]
    fn contains(&self, position: Position<DIMENSIONS>) -> bool {
        let (s_min_x, s_min_y, _s_min_z) = self.min.absolute_position();
        let (s_max_x, s_max_y, _s_max_z) = self.max.absolute_position();

        let (o_x, o_y, _o_z) = position.absolute_position();
        s_min_x <= o_x && s_max_x > o_x && s_min_y <= o_y && s_max_y > o_y
    }

    /// returns an iterator over all of the points
    #[inline]
    fn get_positions(&self) -> HashSet<Position<DIMENSIONS>> { self.iter().collect() }

    #[inline]
    fn boxed_iter(&self) -> BoxedShapeIter<DIMENSIONS> { Box::new(self.into_iter()) }
}

impl<const DIMENSIONS: UVec2> IntoIterator for GridRectangle<DIMENSIONS> {
    type IntoIter = GridRectIter<DIMENSIONS>;
    type Item = Position<DIMENSIONS>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { GridRectIter::new(self.min, self.max) }
}

impl<const DIMENSIONS: UVec2> ShapeIter<DIMENSIONS> for GridRectangle<DIMENSIONS> {
    type Iterator = GridRectIter<DIMENSIONS>;

    #[inline]
    fn iter(&self) -> Self::Iterator { self.into_iter() }
}

impl<const DIMENSIONS: UVec2> From<GridRectangle<DIMENSIONS>> for BoxedShape<DIMENSIONS> {
    fn from(value: GridRectangle<DIMENSIONS>) -> Self { Box::new(value) }
}
