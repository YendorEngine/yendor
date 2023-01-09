use super::DirectionType;

/// The raw `data` behind a Direction is defined here.
pub struct DirectionFlags;

#[rustfmt::skip]
impl DirectionFlags {
    /// `North` = `0b0000_000X`
    pub const NORTH: DirectionType       = 1 << 0;
    /// `East` = `0b0000_00X0`
    pub const EAST: DirectionType        = 1 << 1;
    /// `South` = `0b0000_0X00`
    pub const SOUTH: DirectionType       = 1 << 2;
    /// `West` = `0b0000_X000`
    pub const WEST: DirectionType        = 1 << 3;
    /// `Up` = `0b000X_0000`
    pub const UP: DirectionType          = 1 << 4;
    /// `Down` = `0b00X0_0000`
    pub const DOWN: DirectionType        = 1 << 5;
}
