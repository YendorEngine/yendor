use crate::prelude::*;

/// [`Octant`] represents which octant a [`Position`] is relative to another [`Position`].
///
/// `East` is considered 0deg. The octants are:
/// ```
/// 1: 0..=45
/// 2: 45..=90
/// 3: 90..=135
/// 4: 135..=180
/// 5: 180..=225
/// 6: 225..=270
/// 7: 270..=315
/// 8: 315..=359
/// ```
#[derive(Debug, Clone)]
pub struct Octant<const DIMENSIONS: UVec2>(pub u8);
// adapted from <http://codereview.stackexchange.com/a/95551>

impl<const DIMENSIONS: UVec2> Octant<DIMENSIONS> {
    /// converts a `Position` into a coordinate relative `Octant(0)` offset
    #[inline]
    pub fn to_offset(&self, position: Position<DIMENSIONS>) -> (i64, i64) {
        let offset = position.absolute_position();
        match self.0 {
            0 => (offset.0, offset.1),
            1 => (offset.1, offset.0),
            2 => (offset.1, -offset.0),
            3 => (-offset.0, offset.1),
            4 => (-offset.0, -offset.1),
            5 => (-offset.1, -offset.0),
            6 => (-offset.1, offset.0),
            7 => (offset.0, -offset.1),
            _ => unreachable!(),
        }
    }

    /// converts from a `Octant(0)` relative coordinate into a `Position`
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    pub fn from_offset(&self, offset: (i64, i64), z: i32) -> Position<DIMENSIONS> {
        let p = match self.0 {
            0 => (offset.0, offset.1),
            1 => (offset.1, offset.0),
            2 => (-offset.1, offset.0),
            3 => (-offset.0, offset.1),
            4 => (-offset.0, -offset.1),
            5 => (-offset.1, -offset.0),
            6 => (offset.1, -offset.0),
            7 => (offset.0, -offset.1),
            _ => unreachable!(),
        };
        Position::from_absolute_position((p.0, p.1, z))
    }

    #[inline]
    pub fn octant(&self) -> u8 { self.0 }
}
