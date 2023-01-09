use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct RectIter {
    offset: IVec2,
    max_offset: IVec2,

    /// The minimum corner point of the rect.
    pub min: IVec2,
}

impl RectIter {
    pub fn new(min: IVec2, max: IVec2) -> Self {
        let size = max - min;
        Self {
            min,
            max_offset: size,
            offset: IVec2::ZERO,
        }
    }
}

impl Iterator for RectIter {
    type Item = IVec2;

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

impl From<Rectangle> for RectIter {
    fn from(rect: Rectangle) -> Self { rect.into_iter() }
}
