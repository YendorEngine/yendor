use std::fmt::Display;

use super::*;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Line<const DIM: UVec2> {
    end: Position<DIM>,
    start: Position<DIM>,
}

impl<const DIM: UVec2> Line<DIM> {
    #[inline(always)]
    pub const fn new(start: Position<DIM>, end: Position<DIM>) -> Self { Self { start, end } }

    #[allow(dead_code)]
    #[inline]
    fn iter_exlusive(&self) -> BresenhamLineIter<DIM> { BresenhamLineIter::new(self.start, self.end) }
}

impl<const DIM: UVec2> Shape<DIM> for Line<DIM> {
    #[inline]
    fn get_count(&self) -> u32 { self.start.distance(self.end) }

    #[inline]
    fn contains(&self, position: Position<DIM>) -> bool { self.get_positions().contains(&position) }

    #[inline]
    fn get_positions(&self) -> HashSet<Position<DIM>> { self.iter().collect() }

    #[inline]
    fn boxed_iter(&self) -> BoxedShapeIter<DIM> { Box::new(self.into_iter()) }
}

impl<const DIM: UVec2> ShapeIter<DIM> for Line<DIM> {
    type Iterator = BresenhamLineInclusiveIter<DIM>;

    #[inline]
    fn iter(&self) -> Self::Iterator { self.into_iter() }
}

impl<const DIM: UVec2> IntoIterator for Line<DIM> {
    type IntoIter = BresenhamLineInclusiveIter<DIM>;
    type Item = Position<DIM>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { BresenhamLineInclusiveIter::new(self.start, self.end) }
}

impl<const DIM: UVec2> Display for Line<DIM> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line {{Start: {}, End: {}}}", self.start, self.end)
    }
}

impl<const DIM: UVec2> From<Line<DIM>> for BoxedShape<DIM> {
    fn from(value: Line<DIM>) -> Self { Box::new(value) }
}
