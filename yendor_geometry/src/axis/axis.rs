use crate::prelude::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum GridAxis {
    X,
    Y,
}

impl GridAxis {
    #[inline]
    pub const fn other(self) -> Self {
        match self {
            Self::X => Self::Y,
            Self::Y => Self::X,
        }
    }

    #[inline(always)]
    pub fn new_coord<P: Point>(self, this_axis: i32, other_axis: i32) -> IVec2 {
        match self {
            Self::X => P::new(this_axis, other_axis),
            Self::Y => P::new(other_axis, this_axis),
        }
    }

    pub fn try_new_size(self, this_axis: u32, other_axis: u32) -> UVec2 {
        match self {
            Self::X => UVec2::new(this_axis, other_axis),
            Self::Y => UVec2::new(other_axis, this_axis),
        }
    }

    #[inline]
    pub const fn new_size(self, this_axis: u32, other_axis: u32) -> UVec2 {
        match self {
            Self::X => UVec2::new(this_axis, other_axis),
            Self::Y => UVec2::new(other_axis, this_axis),
        }
    }

    #[inline]
    pub fn size<D: Dimensions>(self, dimensions: D) -> u32 {
        match self {
            Self::X => dimensions.width(),
            Self::Y => dimensions.height(),
        }
    }
}
