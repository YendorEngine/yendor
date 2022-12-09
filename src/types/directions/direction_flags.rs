use super::DirectionType;

#[rustfmt::skip]
pub(crate) struct  DirectionFlags;

impl DirectionFlags {
    pub const NORTH: DirectionType       = 1 << 0;
    pub const EAST: DirectionType        = 1 << 1;
    pub const SOUTH: DirectionType       = 1 << 2;
    pub const WEST: DirectionType        = 1 << 3;
    pub const UP: DirectionType          = 1 << 4;
    pub const DOWN: DirectionType        = 1 << 5;
}