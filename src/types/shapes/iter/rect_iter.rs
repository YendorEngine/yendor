use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GridRectIter {
    offset: IVec2,
    max_offset: IVec2,

    /// The minimum corner point of the rect.
    pub min: Position,
}

impl GridRectIter {
    pub fn new(min: Position, max: Position) -> Self {
        let size = max.gridpoint() - min.gridpoint();
        Self {
            min,
            offset: IVec2::ZERO,
            max_offset: size.as_ivec2(),
        }
    }
}

impl Iterator for GridRectIter {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset.y > self.max_offset.y {
            return None;
        }

        let p = self.offset;
        self.offset.x += 1;
        if self.offset.x > self.max_offset.x {
            self.offset.x = 0;
            self.offset.y += 1;
        }

        Some(self.min + p)
    }
}
