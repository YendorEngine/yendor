use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct RectIter {
    offset: UVec2,
    max_offset: UVec2,

    /// The minimum corner point of the rect.
    pub min: ChunkPosition,
}

impl RectIter {
    pub fn new(min: ChunkPosition, max: ChunkPosition) -> Self {
        let (min_x, min_y, _) = min.as_absolute();
        let (max_x, max_y, _) = max.as_absolute();

        let size_x = (max_x - min_x) as u32;
        let size_y = (max_y - min_y) as u32;
        Self {
            min,
            offset: UVec2::ZERO,
            max_offset: UVec2::new(size_x, size_y),
        }
    }
}

impl Iterator for RectIter {
    type Item = ChunkPosition;

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
