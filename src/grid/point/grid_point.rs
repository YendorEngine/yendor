use crate::prelude::*;

pub trait GridPoint: Clone + Copy {
    fn x_int32(&self) -> i32;

    fn y_int32(&self) -> i32;

    fn x_uint32(&self) -> u32;

    fn y_uint32(&self) -> u32;

    fn x_float32(&self) -> f32;

    fn y_float32(&self) -> f32;

    #[inline]
    fn size(&self) -> usize { (self.x_int32() * self.y_int32()) as usize }

    /// Get the point's corresponding 1d index.
    #[inline(always)]
    fn as_index_unchecked<I: TryInto<usize>>(&self, width: I) -> usize {
        let width = width.try_into().unwrap_or_else(|_v| panic!("width is too large"));
        self.y_int32() as usize * width + self.x_int32() as usize
    }

    #[inline(always)]
    fn as_index(&self, size: UVec2) -> Option<usize> {
        if self.is_valid(size) { Some(self.as_index_unchecked(size.x)) } else { None }
    }

    /// Returns true if the point is valid for the given size.
    #[inline]
    fn is_valid(&self, size: UVec2) -> bool {
        let x = self.x_int32();
        let y = self.y_int32();
        x >= 0 && y >= 0 && (x as u32) < size.x && (y as u32) < size.y
    }

    #[inline]
    fn mid_point(&self, point: impl GridPoint) -> IVec2 {
        IVec2 {
            x: (self.x_int32() + point.x_int32()) / 2,
            y: (self.y_int32() + point.y_int32()) / 2,
        }
    }

    #[inline]
    fn distance(&self, point: impl GridPoint) -> u32 {
        let start = UVec2::new(self.x_uint32(), self.y_uint32());
        let end = UVec2::new(point.x_uint32(), point.y_uint32());
        start.sub(end).max_element()
    }

    /// Returns an iterator over all points within the size.
    fn iter(self) -> PointIterRowMajor {
        PointIterRowMajor::new(UVec2::new(self.x_uint32(), self.y_uint32()))
    }
}

impl GridPoint for IVec2 {
    fn x_int32(&self) -> i32 { self.x }

    fn y_int32(&self) -> i32 { self.y }

    fn x_uint32(&self) -> u32 { self.x as u32 }

    fn y_uint32(&self) -> u32 { self.y as u32 }

    fn x_float32(&self) -> f32 { self.x as f32 }

    fn y_float32(&self) -> f32 { self.y as f32 }
}

impl GridPoint for UVec2 {
    fn x_int32(&self) -> i32 { self.x as i32 }

    fn y_int32(&self) -> i32 { self.y as i32 }

    fn x_uint32(&self) -> u32 { self.x }

    fn y_uint32(&self) -> u32 { self.y }

    fn x_float32(&self) -> f32 { self.x as f32 }

    fn y_float32(&self) -> f32 { self.y as f32 }
}

impl GridPoint for Vec2 {
    fn x_int32(&self) -> i32 { self.x.floor() as i32 }

    fn y_int32(&self) -> i32 { self.y.floor() as i32 }

    fn x_uint32(&self) -> u32 { self.x.floor() as u32 }

    fn y_uint32(&self) -> u32 { self.y.floor() as u32 }

    fn x_float32(&self) -> f32 { self.x }

    fn y_float32(&self) -> f32 { self.y }
}
