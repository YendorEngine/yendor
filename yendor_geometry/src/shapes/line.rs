use std::fmt::Display;

use super::*;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Line<const DIMENSIONS: UVec2> {
    end: Position<DIMENSIONS>,
    start: Position<DIMENSIONS>,
}

impl<const DIMENSIONS: UVec2> Line<DIMENSIONS> {
    #[inline(always)]
    pub const fn new(start: Position<DIMENSIONS>, end: Position<DIMENSIONS>) -> Self { Self { start, end } }

    #[allow(dead_code)]
    #[inline]
    fn iter_exlusive(&self) -> BresenhamLineIter<DIMENSIONS> { BresenhamLineIter::new(self.start, self.end) }
}

impl<const DIMENSIONS: UVec2> Shape<DIMENSIONS> for Line<DIMENSIONS> {
    #[inline]
    fn get_count(&self) -> u32 { self.start.distance(self.end) }

    #[inline]
    fn contains(&self, position: Position<DIMENSIONS>) -> bool { self.get_positions().contains(&position) }

    #[inline]
    fn get_positions(&self) -> HashSet<Position<DIMENSIONS>> { self.iter().collect() }

    #[inline]
    fn boxed_iter(&self) -> BoxedShapeIter<DIMENSIONS> { Box::new(self.into_iter()) }
}

impl<const DIMENSIONS: UVec2> ShapeIter<DIMENSIONS> for Line<DIMENSIONS> {
    type Iterator = BresenhamLineInclusiveIter<DIMENSIONS>;

    #[inline]
    fn iter(&self) -> Self::Iterator { self.into_iter() }
}

impl<const DIMENSIONS: UVec2> IntoIterator for Line<DIMENSIONS> {
    type IntoIter = BresenhamLineInclusiveIter<DIMENSIONS>;
    type Item = Position<DIMENSIONS>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { BresenhamLineInclusiveIter::new(self.start, self.end) }
}

impl<const DIMENSIONS: UVec2> Display for Line<DIMENSIONS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line {{Start: {}, End: {}}}", self.start, self.end)
    }
}

impl<const DIMENSIONS: UVec2> From<Line<DIMENSIONS>> for BoxedShape<DIMENSIONS> {
    fn from(value: Line<DIMENSIONS>) -> Self { Box::new(value) }
}
