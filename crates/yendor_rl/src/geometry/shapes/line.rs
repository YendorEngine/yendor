use crate::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Line {
    end: IVec2,
    start: IVec2,
}

impl Line {
    #[inline(always)]
    pub const fn new(start: IVec2, end: IVec2) -> Self {
        Self { start, end }
    }

    #[allow(dead_code)]
    #[inline]
    fn iter_exlusive(&self) -> BresenhamLineIter {
        BresenhamLineIter::new(self.start, self.end)
    }
}

impl Shape for Line {
    #[inline]
    fn get_count(&self) -> u32 {
        (self.end - self.start).abs().max_element() as u32
    }

    #[inline]
    fn contains(&self, position: IVec2) -> bool {
        self.get_positions().contains(&position)
    }

    #[inline]
    fn get_positions(&self) -> HashSet<IVec2> {
        self.iter().collect()
    }

    #[inline]
    fn boxed_iter(&self) -> BoxedShapeIter {
        Box::new(self.into_iter())
    }
}

impl ShapeIter for Line {
    type Iterator = BresenhamLineInclusiveIter;

    #[inline]
    fn iter(&self) -> Self::Iterator {
        self.into_iter()
    }
}

impl IntoIterator for Line {
    type IntoIter = BresenhamLineInclusiveIter;
    type Item = IVec2;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        BresenhamLineInclusiveIter::new(self.start, self.end)
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line {{Start: {}, End: {}}}", self.start, self.end)
    }
}

impl From<Line> for BoxedShape {
    fn from(value: Line) -> Self {
        Box::new(value)
    }
}
