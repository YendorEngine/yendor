#![allow(dead_code)]

use super::*;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Rectangle {
    pub min: ChunkPosition,
    pub max: ChunkPosition,
}

impl Rectangle {
    #[inline]
    pub fn new(min: ChunkPosition, max: ChunkPosition) -> Self {
        let (min, max) = Self::find_min_max(min, max);
        Self { min, max }
    }

    fn find_min_max(min: ChunkPosition, max: ChunkPosition) -> (ChunkPosition, ChunkPosition) {
        let (abs_min_x, abs_min_y, abs_min_z) = min.as_absolute();
        let (abs_max_x, abs_max_y, abs_max_z) = max.as_absolute();

        let min_x = abs_min_x.min(abs_max_x);
        let max_x = abs_min_x.max(abs_max_x);

        let min_y = abs_min_y.min(abs_max_y);
        let max_y = abs_min_y.max(abs_max_y);

        let min_z = abs_min_z.min(abs_max_z);
        let max_z = abs_min_z.max(abs_max_z);

        (
            ChunkPosition::from_raw(min_x, min_y, min_z),
            ChunkPosition::from_raw(max_x, max_y, max_z),
        )
    }
}

impl Rectangle {
    #[inline]
    pub fn width(&self) -> u32 { self.max.distance_x(self.min).expect("Error getting distance!") }

    #[inline]
    pub fn height(&self) -> u32 { self.max.distance_y(self.min).expect("Error getting distance!") }

    #[inline]
    pub const fn min(&self) -> ChunkPosition { self.min }

    #[inline]
    pub const fn max(&self) -> ChunkPosition { self.max }

    #[inline]
    pub fn is_square(&self) -> bool { self.width() == self.height() }
}

impl Rectangle {
    #[inline]
    fn center(&self) -> ChunkPosition { self.min.lerp(self.max, 0.5) }

    #[inline]
    fn left(&self) -> ChunkPosition { self.center() - IVec2::new((self.width() / 2) as i32, 0) }

    #[inline]
    fn right(&self) -> ChunkPosition { self.center() + IVec2::new((self.width() / 2) as i32, 0) }

    #[inline]
    fn top(&self) -> ChunkPosition { self.center() + IVec2::new(0, (self.height() / 2) as i32) }

    #[inline]
    fn bottom(&self) -> ChunkPosition { self.center() - IVec2::new(0, (self.height() / 2) as i32) }

    /// Check if this rectangle intersects another rectangle.
    #[inline]
    pub fn intersects(&self, other: Self) -> bool {
        let (s_min_x, s_min_y, _s_min_z) = self.min.as_absolute();
        let (s_max_x, s_max_y, _s_max_z) = self.max.as_absolute();

        let (o_min_x, o_min_y, _o_min_z) = other.min.as_absolute();
        let (o_max_x, o_max_y, _o_max_z) = other.max.as_absolute();
        // (self.min.cmple(other.max) & self.max.cmpge(other.min)).all()
        s_min_x <= o_max_x && s_max_x >= o_min_x && s_min_y <= o_max_y && s_max_y >= o_min_y
    }

    /// Calls a function for each x/y point in the rectangle
    pub fn for_each<F>(&self, f: F)
    where F: FnMut(ChunkPosition) {
        self.into_iter().for_each(f)
    }
}

impl Shape for Rectangle {
    #[inline]
    fn get_count(&self) -> u32 { self.width() * self.height() }

    #[inline]
    fn contains(&self, position: ChunkPosition) -> bool {
        let (s_min_x, s_min_y, _s_min_z) = self.min.as_absolute();
        let (s_max_x, s_max_y, _s_max_z) = self.max.as_absolute();

        let (o_x, o_y, _o_z) = position.as_absolute();
        s_min_x <= o_x && s_max_x > o_x && s_min_y <= o_y && s_max_y > o_y
    }

    /// returns an iterator over all of the points
    #[inline]
    fn get_positions(&self) -> HashSet<ChunkPosition> { self.iter().collect() }

    #[inline]
    fn boxed_iter(&self) -> BoxedShapeIter { Box::new(self.into_iter()) }
}

impl IntoIterator for Rectangle {
    type IntoIter = RectIter;
    type Item = ChunkPosition;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { RectIter::new(self.min, self.max) }
}

impl ShapeIter for Rectangle {
    type Iterator = RectIter;

    #[inline]
    fn iter(&self) -> Self::Iterator { self.into_iter() }
}

impl From<Rectangle> for BoxedShape {
    fn from(value: Rectangle) -> Self { Box::new(value) }
}
