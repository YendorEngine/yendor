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
    pub fn new(x: i64, y: i64, z: i32) -> Self { Self { x, y, z } }
}

// Getters
//##################
impl ChunkOffset {
    pub fn x(&self) -> i64 { self.x }

    pub fn y(&self) -> i64 { self.y }

    pub fn z(&self) -> i32 { self.z }
}

// Implementations
//##################
impl Add<IVec2> for ChunkOffset {
    type Output = Self;

    fn add(self, rhs: IVec2) -> Self::Output {
        Self::new(self.x + rhs.x as i64, self.y + rhs.y as i64, self.z)
    }
}

impl Add<UVec2> for ChunkOffset {
    type Output = Self;

    fn add(self, rhs: UVec2) -> Self::Output {
        Self::new(self.x + rhs.x as i64, self.y + rhs.y as i64, self.z)
    }
}

impl AddAssign<IVec2> for ChunkOffset {
    fn add_assign(&mut self, rhs: IVec2) {
        self.x += rhs.x as i64;
        self.y += rhs.y as i64;
    }
}

impl AddAssign<UVec2> for ChunkOffset {
    fn add_assign(&mut self, rhs: UVec2) {
        self.x += rhs.x as i64;
        self.y += rhs.y as i64;
    }
}

impl Sub<IVec2> for ChunkOffset {
    type Output = Self;

    fn sub(self, rhs: IVec2) -> Self::Output {
        Self::new(self.x - rhs.x as i64, self.y - rhs.y as i64, self.z)
    }
}

impl Sub<UVec2> for ChunkOffset {
    type Output = Self;

    fn sub(self, rhs: UVec2) -> Self::Output {
        Self::new(self.x - rhs.x as i64, self.y - rhs.y as i64, self.z)
    }
}

impl SubAssign<IVec2> for ChunkOffset {
    fn sub_assign(&mut self, rhs: IVec2) {
        self.x -= rhs.x as i64;
        self.y -= rhs.y as i64;
    }
}

impl SubAssign<UVec2> for ChunkOffset {
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
