use crate::prelude::*;

////////////////////////////////////////////////////////////
// Dimensions
////////////////////////////////////////////////////////////
const MAX_SIZE: u32 = i32::MAX as u32;

#[derive(Debug)]
pub struct DimensionTooLargeForSize;

const fn check_size(value: u32) -> bool {
    value <= MAX_SIZE
}

/// A trait for types representing a 2d dimension.
#[allow(clippy::new_ret_no_self)]
pub trait Dimensions: Clone + Copy {
    // Safely create a new UVec2
    fn new_try(width: u32, height: u32) -> Option<UVec2> {
        if check_size(width) && check_size(height) {
            Some(UVec2::new(width, height))
        } else {
            None
        }
    }

    // Create a new UVec2
    // Panics if `width` or `height` is greater than `i32::MAX`
    fn new(width: u32, height: u32) -> UVec2 {
        Self::new_try(width, height).map_or_else(
            || panic!("Size is too big: ({width}, {height}). Max is {MAX_SIZE}."),
            |size| size,
        )
    }

    /// Returns width value.
    fn width(&self) -> u32;

    /// Returns height value.
    fn height(&self) -> u32;

    #[inline]
    fn count(&self) -> usize {
        (self.width() * self.height()) as usize
    }

    /// Convert dimensions to UVec2 (u32).
    #[inline]
    fn as_uvec2(&self) -> UVec2 {
        UVec2::new(self.width(), self.height())
    }

    /// Convert dimensions to IVec2 (i32).
    #[inline]
    fn as_ivec2(&self) -> IVec2 {
        self.as_uvec2().as_ivec2()
    }

    /// Convert dimensions to `Vec2` (f32).
    #[inline]
    fn as_vec2(&self) -> Vec2 {
        self.as_uvec2().as_vec2()
    }

    /// Convert dimensions to `[i32;2]`.
    #[inline]
    fn as_array(&self) -> [u32; 2] {
        self.as_uvec2().to_array()
    }

    /// Returns true if the point is valid within the size.
    #[inline]
    fn contains(&self, point: impl Point) -> bool {
        (point.x_uint32()) < self.width() && (point.y_uint32()) < self.height()
    }

    /// Returns an iterator over all points within the size.
    fn iter(self) -> PointIterRowMajor {
        PointIterRowMajor::new(self)
    }
}

#[macro_export]
macro_rules! impl_size2d_array {
    ($type:ty) => {
        impl Dimensions for $type {
            fn width(&self) -> u32 {
                self[0] as u32
            }

            fn height(&self) -> u32 {
                self[1] as u32
            }
        }
    };
}

#[macro_export]
macro_rules! impl_size2d_tuple {
    ($type:ty) => {
        impl Dimensions for $type {
            fn width(&self) -> u32 {
                self.0 as u32
            }

            fn height(&self) -> u32 {
                self.1 as u32
            }
        }
    };
}

impl_size2d_array!(IVec2);
impl_size2d_array!(UVec2);
impl_size2d_array!([u32; 2]);
impl_size2d_array!([i32; 2]);
impl_size2d_array!([usize; 2]);
