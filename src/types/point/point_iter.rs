use crate::prelude::*;

////////////////////////////////////////////////////////////
// Point Iter
////////////////////////////////////////////////////////////
pub struct PointIterRowMajor {
    coord: IVec2,
    size: UVec2,
}

impl PointIterRowMajor {
    pub fn new(size: impl Dimensions) -> Self {
        Self {
            size: size.as_uvec2(),
            coord: IVec2::new(0, 0),
        }
    }
}

impl Iterator for PointIterRowMajor {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.coord.y == self.size.height() as i32 {
            return None;
        }
        let coord = self.coord;
        self.coord.x += 1;
        if self.coord.x == self.size.width() as i32 {
            self.coord.x = 0;
            self.coord.y += 1;
        }
        Some(coord)
    }
}

////////////////////////////////////////////////////////////
// Adjacent Iter
////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct AdjIterator<'a> {
    i: usize,
    p: IVec2,
    arr: &'a [Direction],
}

impl<'a> AdjIterator<'a> {
    pub fn new(p: impl Point, arr: &'a [Direction]) -> Self {
        Self {
            i: 0,
            p: p.as_ivec2(),
            arr,
        }
    }
}

impl<'a> Iterator for AdjIterator<'a> {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.arr.len() {
            return None;
        };
        let p = self.p + self.arr[self.i].coord();
        self.i += 1;
        Some(p)
    }
}
