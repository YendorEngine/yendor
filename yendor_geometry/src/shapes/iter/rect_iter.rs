use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GridRectIter<const DIM: UVec2> {
    offset: IVec2,
    max_offset: IVec2,

    /// The minimum corner point of the rect.
    pub min: Position<DIM>,
}

impl<const DIM: UVec2> GridRectIter<DIM> {
    pub fn new(min: Position<DIM>, max: Position<DIM>) -> Self {
        let size = max.gridpoint() - min.gridpoint();
        Self {
            min,
            offset: IVec2::ZERO,
            max_offset: size.as_ivec2(),
        }
    }
}

impl<const DIM: UVec2> Iterator for GridRectIter<DIM> {
    type Item = Position<DIM>;

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
