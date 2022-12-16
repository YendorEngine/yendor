use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkOffset {
    pub x: i64,
    pub y: i64,
    pub z: i32,
}

// Constructor
//##################
impl ChunkOffset {
    #[inline(always)]
    pub fn new(x: i64, y: i64, z: i32) -> Self { Self { x, y, z } }

    #[inline(always)]
    pub fn new_xy(x: i64, y: i64) -> Self { Self::new(x, y, 0) }
}

// Getters
//##################
impl ChunkOffset {
    #[inline]
    pub fn x(&self) -> i64 { self.x }

    #[inline]
    pub fn y(&self) -> i64 { self.y }

    #[inline]
    pub fn z(&self) -> i32 { self.z }
}

// Implementations
//##################
impl Add<IVec2> for ChunkOffset {
    type Output = Self;

    #[inline]
    fn add(self, rhs: IVec2) -> Self::Output {
        Self::new(self.x + rhs.x as i64, self.y + rhs.y as i64, self.z)
    }
}

impl Add<UVec2> for ChunkOffset {
    type Output = Self;

    #[inline]
    fn add(self, rhs: UVec2) -> Self::Output {
        Self::new(self.x + rhs.x as i64, self.y + rhs.y as i64, self.z)
    }
}

impl AddAssign<IVec2> for ChunkOffset {
    #[inline]
    fn add_assign(&mut self, rhs: IVec2) {
        self.x += rhs.x as i64;
        self.y += rhs.y as i64;
    }
}

impl AddAssign<UVec2> for ChunkOffset {
    #[inline]
    fn add_assign(&mut self, rhs: UVec2) {
        self.x += rhs.x as i64;
        self.y += rhs.y as i64;
    }
}

impl Sub<IVec2> for ChunkOffset {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: IVec2) -> Self::Output {
        Self::new(self.x - rhs.x as i64, self.y - rhs.y as i64, self.z)
    }
}

impl Sub<UVec2> for ChunkOffset {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: UVec2) -> Self::Output {
        Self::new(self.x - rhs.x as i64, self.y - rhs.y as i64, self.z)
    }
}

impl SubAssign<IVec2> for ChunkOffset {
    #[inline]
    fn sub_assign(&mut self, rhs: IVec2) {
        self.x -= rhs.x as i64;
        self.y -= rhs.y as i64;
    }
}

impl SubAssign<UVec2> for ChunkOffset {
    #[inline]
    fn sub_assign(&mut self, rhs: UVec2) {
        self.x -= rhs.x as i64;
        self.y -= rhs.y as i64;
    }
}

impl Display for ChunkOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ChunkPosition {{\n\tx: {},\n\ty: {},\n\tz: {}\n}}",
            self.x, self.y, self.z
        )
    }
}

impl Debug for ChunkOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
