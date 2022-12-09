use std::fmt::Display;

use super::*;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Line<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> {
    end: Position<GRID_WIDTH, GRID_HEIGHT>,
    start: Position<GRID_WIDTH, GRID_HEIGHT>,
}

impl<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> Line<GRID_WIDTH, GRID_HEIGHT> {
    #[inline(always)]
    pub const fn new(
        start: Position<GRID_WIDTH, GRID_HEIGHT>,
        end: Position<GRID_WIDTH, GRID_HEIGHT>,
    ) -> Self {
        Self { start, end }
    }

    #[allow(dead_code)]
    #[inline]
    fn iter_exlusive(&self) -> BresenhamLineIter<GRID_WIDTH, GRID_HEIGHT> {
        BresenhamLineIter::new(self.start, self.end)
    }
}

impl<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> Shape<GRID_WIDTH, GRID_HEIGHT>
    for Line<GRID_WIDTH, GRID_HEIGHT>
{
    #[inline]
    fn get_count(&self) -> u32 {
        self.start.distance(self.end)
    }

    #[inline]
    fn contains(&self, position: Position<GRID_WIDTH, GRID_HEIGHT>) -> bool {
        self.get_positions().contains(&position)
    }

    #[inline]
    fn get_positions(&self) -> HashSet<Position<GRID_WIDTH, GRID_HEIGHT>> {
        self.iter().collect()
    }

    #[inline]
    fn boxed_iter(&self) -> BoxedShapeIter<GRID_WIDTH, GRID_HEIGHT> {
        Box::new(self.into_iter())
    }
}

impl<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> ShapeIter<GRID_WIDTH, GRID_HEIGHT>
    for Line<GRID_WIDTH, GRID_HEIGHT>
{
    type Iterator = BresenhamLineInclusiveIter<GRID_WIDTH, GRID_HEIGHT>;

    #[inline]
    fn iter(&self) -> Self::Iterator {
        self.into_iter()
    }
}

impl<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> IntoIterator for Line<GRID_WIDTH, GRID_HEIGHT> {
    type IntoIter = BresenhamLineInclusiveIter<GRID_WIDTH, GRID_HEIGHT>;
    type Item = Position<GRID_WIDTH, GRID_HEIGHT>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        BresenhamLineInclusiveIter::new(self.start, self.end)
    }
}

impl<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> Display for Line<GRID_WIDTH, GRID_HEIGHT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line {{Start: {}, End: {}}}", self.start, self.end)
    }
}

impl<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> From<Line<GRID_WIDTH, GRID_HEIGHT>>
    for BoxedShape<GRID_WIDTH, GRID_HEIGHT>
{
    fn from(value: Line<GRID_WIDTH, GRID_HEIGHT>) -> Self {
        Box::new(value)
    }
}
