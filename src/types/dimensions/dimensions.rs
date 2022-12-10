use crate::prelude::*;

////////////////////////////////////////////////////////////
// Dimensions
////////////////////////////////////////////////////////////

/// A trait for types representing a 2d dimension.
pub trait Dimensions: Clone + Copy {
    /// Returns width value.
    fn width(&self) -> u32;

    /// Returns height value.
    fn height(&self) -> u32;

    #[inline]
    fn len(&self) -> usize { (self.width() * self.height()) as usize }

    /// Convert dimensions to UVec2 (u32).
    #[inline]
    fn as_uvec2(&self) -> UVec2 { UVec2::new(self.width(), self.height()) }

    /// Convert dimensions to `[i32;2]`.
    #[inline]
    fn as_array(&self) -> [u32; 2] { self.as_uvec2().to_array() }

    /// Returns true if the point is valid within the size.
    #[inline]
    fn contains(&self, point: impl Point) -> bool {
        (point.x_uint32()) < self.width() && (point.y_uint32()) < self.height()
    }

    /// Returns an iterator over all points within the size.
    fn iter(self) -> PointIterRowMajor { PointIterRowMajor::new(self) }
}

#[macro_export]
macro_rules! impl_size2d_array {
    ($type:ty) => {
        impl Dimensions for $type {
            fn width(&self) -> u32 { self[0] as u32 }

            fn height(&self) -> u32 { self[1] as u32 }
        }
    };
}

#[macro_export]
macro_rules! impl_size2d_tuple {
    ($type:ty) => {
        impl Dimensions for $type {
            fn width(&self) -> u32 { self.0 as u32 }

            fn height(&self) -> u32 { self.1 as u32 }
        }
    };
}

impl_size2d_array!(IVec2);
impl_size2d_array!(UVec2);
impl_size2d_array!([u32; 2]);
impl_size2d_array!([i32; 2]);
impl_size2d_array!([usize; 2]);
