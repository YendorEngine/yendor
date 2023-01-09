use crate::prelude::*;

////////////////////////////////////////////////////////////
// Point Iter
////////////////////////////////////////////////////////////
pub struct PointIterRowMajor {
    coord: IVec2,
    size: UVec2,
}

impl PointIterRowMajor {
    pub const fn new(size: UVec2) -> Self {
        Self {
            size,
            coord: IVec2::ZERO,
        }
    }
}

impl Iterator for PointIterRowMajor {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.coord.y == self.size.x as i32 {
            return None;
        }
        let coord = self.coord;
        self.coord.x += 1;
        if self.coord.x == self.size.y as i32 {
            self.coord.x = 0;
            self.coord.y += 1;
        }
        Some(coord)
    }
}

////////////////////////////////////////////////////////////
// Adjacent Iter
////////////////////////////////////////////////////////////
pub struct AdjIterator {
    i: usize,
    p: IVec2,
    count: usize,
    dir_iter: DirectionIter,
}

impl AdjIterator {
    pub fn new(p: IVec2, dir_iter: DirectionIter) -> Self {
        Self {
            p,
            i: 0,
            dir_iter,
            count: dir_iter.count(),
        }
    }
}

impl Iterator for AdjIterator {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.count {
            return None;
        };

        let p = self.p + self.dir_iter.next().unwrap().coord();
        self.i += 1;
        Some(p)
    }
}
