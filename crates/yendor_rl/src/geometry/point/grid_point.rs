use crate::prelude::*;

pub trait GridPoint: Clone + Copy {
    fn x(&self) -> i32;

    fn y(&self) -> i32;

    #[inline]
    fn as_ivec2(&self) -> IVec2 { IVec2::new(self.x(), self.y()) }

    #[inline]
    fn as_uvec2(&self) -> UVec2 { self.as_ivec2().as_uvec2() }

    #[inline]
    fn as_vec2(&self) -> Vec2 { self.as_ivec2().as_vec2() }

    #[inline]
    fn size(&self) -> usize { (self.x() * self.y()) as usize }

    /// Get the point's corresponding 1d index.
    #[inline(always)]
    fn as_index_unchecked<I: TryInto<usize>>(&self, width: I) -> usize {
        let width = width.try_into().unwrap_or_else(|_v| panic!("width is too large"));
        self.y() as usize * width + self.x() as usize
    }

    #[inline(always)]
    fn as_index(&self, size: UVec2) -> Option<usize> {
        if self.is_valid(size) { Some(self.as_index_unchecked(size.x)) } else { None }
    }

    /// Returns true if the point is valid for the given size.
    #[inline]
    fn is_valid(&self, size: UVec2) -> bool {
        let x = self.x();
        let y = self.y();
        x >= 0 && y >= 0 && (x as u32) < size.x && (y as u32) < size.y
    }

    #[inline]
    fn mid_point(&self, point: impl GridPoint) -> IVec2 {
        IVec2 {
            x: (self.x() + point.x()) / 2,
            y: (self.y() + point.y()) / 2,
        }
    }

    /// Chevbyshev distance between two points.
    #[inline]
    fn distance(&self, point: impl GridPoint) -> u32 {
        let start = self.as_uvec2();
        let end = point.as_uvec2();
        start.sub(end).max_element()
    }

    /// Returns an iterator over all points within the size.
    fn iter(self) -> PointIterRowMajor { PointIterRowMajor::new(self.as_uvec2()) }
}

macro_rules! impl_grid_point {
    ($type:ty) => {
        impl GridPoint for $type {
            fn x(&self) -> i32 { self[0] as i32 }

            fn y(&self) -> i32 { self[1] as i32 }
        }
    };
}

impl_grid_point!(IVec2);
impl_grid_point!(UVec2);
impl_grid_point!([u32; 2]);
impl_grid_point!([i32; 2]);
impl_grid_point!([usize; 2]);

impl GridPoint for Vec2 {
    fn x(&self) -> i32 { self.x.floor() as i32 }

    fn y(&self) -> i32 { self.y.floor() as i32 }
}
