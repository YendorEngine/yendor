use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct ChunkPosition {
    x: i64,
    y: i64,
    z: i32,
}

// Constructor
//##################
impl ChunkPosition {
    pub const ZERO: Self = Self::new(0, 0, 0);

    #[inline(always)]
    pub const fn new(x: i64, y: i64, z: i32) -> Self { Self { x, y, z } }

    #[inline(always)]
    pub const fn from_absolute(x: i64, y: i64, z: i32) -> Self { Self::new(x, y, z) }
}

// Getters
//##################
impl ChunkPosition {
    /// The x coordinate of the chunk.
    #[inline]
    pub fn abs_x(&self) -> i64 { self.x }

    /// The y coordinate of the chunk.  
    #[inline]
    pub fn abs_y(&self) -> i64 { self.y }

    /// The z coordinate of the chunk.
    #[inline]
    pub fn abs_z(&self) -> i32 { self.z }

    /// Check dimensions first!!!
    #[inline]
    pub fn as_absolute(&self) -> (i64, i64, i32) { (self.x, self.y, self.z) }
}

// Functions
//##################
impl ChunkPosition {
    #[inline]
    pub fn distance(&self, other: Self) -> u32 { self.distance_x(other).max(self.distance_y(other)) }

    #[inline]
    pub fn distance_x(&self, other: Self) -> u32 { (other.abs_x() - self.abs_x()) as u32 }

    #[inline]
    pub fn distance_y(&self, other: Self) -> u32 { (other.abs_y() - self.abs_y()) as u32 }

    pub fn lerp(&self, other: Self, percent: f32) -> Self {
        let x = ((other.abs_x() - self.abs_x()) as f64).mul_add(percent as f64, self.abs_x() as f64) as i64;
        let y = ((other.abs_y() - self.abs_y()) as f64).mul_add(percent as f64, self.abs_y() as f64) as i64;
        let z = ((other.abs_z() - self.abs_z()) as f64).mul_add(percent as f64, self.abs_z() as f64) as i32;

        Self::new(x, y, z)
    }

    pub fn octant_to(&self, other: Self) -> Octant {
        // adapted from <http://codereview.stackexchange.com/a/95551>
        let start = self.as_absolute();
        let end = other.as_absolute();

        let mut dx = end.0 - start.0;
        let mut dy = end.1 - start.1;
        let mut octant = 0;
        if dy < 0 {
            dx = -dx;
            dy = -dy;
            octant += 4;
        }
        if dx < 0 {
            let tmp = dx;
            dx = dy;
            dy = -tmp;
            octant += 2;
        }
        if dx < dy {
            octant += 1;
        }

        Octant(octant)
    }
}

// Implementations
//##################
impl Add<ChunkOffset> for ChunkPosition {
    type Output = Self;

    #[inline]
    fn add(self, rhs: ChunkOffset) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<IVec2> for ChunkPosition {
    type Output = Self;

    #[inline]
    fn add(self, rhs: IVec2) -> Self::Output {
        Self::new(self.x + rhs.x as i64, self.y + rhs.y as i64, self.z)
    }
}

impl Add<UVec2> for ChunkPosition {
    type Output = Self;

    #[inline]
    fn add(self, rhs: UVec2) -> Self::Output {
        Self::new(self.x + rhs.x as i64, self.y + rhs.y as i64, self.z)
    }
}

impl AddAssign<ChunkOffset> for ChunkPosition {
    #[inline]
    fn add_assign(&mut self, rhs: ChunkOffset) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<IVec2> for ChunkPosition {
    #[inline]
    fn add_assign(&mut self, rhs: IVec2) {
        self.x += rhs.x as i64;
        self.y += rhs.y as i64;
    }
}

impl AddAssign<UVec2> for ChunkPosition {
    #[inline]
    fn add_assign(&mut self, rhs: UVec2) {
        self.x += rhs.x as i64;
        self.y += rhs.y as i64;
    }
}

impl Sub<ChunkOffset> for ChunkPosition {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: ChunkOffset) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<IVec2> for ChunkPosition {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: IVec2) -> Self::Output {
        Self::new(self.x - rhs.x as i64, self.y - rhs.y as i64, self.z)
    }
}

impl Sub<UVec2> for ChunkPosition {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: UVec2) -> Self::Output {
        Self::new(self.x - rhs.x as i64, self.y - rhs.y as i64, self.z)
    }
}

impl SubAssign<ChunkOffset> for ChunkPosition {
    #[inline]
    fn sub_assign(&mut self, rhs: ChunkOffset) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl SubAssign<IVec2> for ChunkPosition {
    #[inline]
    fn sub_assign(&mut self, rhs: IVec2) {
        self.x -= rhs.x as i64;
        self.y -= rhs.y as i64;
    }
}

impl SubAssign<UVec2> for ChunkPosition {
    #[inline]
    fn sub_assign(&mut self, rhs: UVec2) {
        self.x -= rhs.x as i64;
        self.y -= rhs.y as i64;
    }
}

impl Display for ChunkPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ChunkPosition {{\n\tx: {},\n\ty: {},\n\tz: {}\n}}",
            self.x, self.y, self.z
        )
    }
}

impl Debug for ChunkPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl ToUVec2 for ChunkPosition {
    #[inline]
    fn as_uvec2(&self) -> UVec2 { UVec2::new(self.x as u32, self.y as u32) }
}

impl From<IVec3> for ChunkPosition {
    #[inline]
    fn from(v: IVec3) -> Self { Self::new(v.x as i64, v.y as i64, v.z) }
}
