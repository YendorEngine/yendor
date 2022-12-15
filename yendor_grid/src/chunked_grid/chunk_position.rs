pub use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
enum DimensionType {
    Dimensions(UVec2),
    Modifier((i64, i64)),
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct ChunkPosition {
    dimensions: DimensionType,
    x: i64,
    y: i64,
    z: i32,
}

// Constructor
//######################
impl ChunkPosition {
    pub fn new(chunk_position: IVec3, local_position: UVec2) -> Self {
        Self {
            dimensions: DimensionType::Modifier((chunk_position.x as i64, chunk_position.y as i64)),
            x: local_position.x as i64,
            y: local_position.y as i64,
            z: chunk_position.z,
        }
    }

    pub fn new_dimensions(chunk_position: IVec3, local_position: UVec2, dimensions: UVec2) -> Self {
        Self {
            dimensions: DimensionType::Dimensions(dimensions),
            x: chunk_position.x as i64 * dimensions.x as i64 + local_position.x as i64,
            y: chunk_position.y as i64 * dimensions.y as i64 + local_position.y as i64,
            z: chunk_position.z,
        }
    }

    pub fn from_raw(x: i64, y: i64, z: i32) -> Self {
        Self {
            dimensions: DimensionType::Modifier((0, 0)),
            x,
            y,
            z,
        }
    }
}

// Position(Dimensions)
//######################
impl ChunkPosition {
    pub fn position(&self, dimensions: UVec2) -> (IVec3, UVec2) {
        let modifier = match self.dimensions {
            DimensionType::Dimensions(_v) => (0i64, 0i64),
            DimensionType::Modifier(m) => m,
        };

        let x = modifier.0 * dimensions.x as i64 + self.x;
        let y = modifier.1 * dimensions.y as i64 + self.y;
        Self::from_absolute_position((x, y, self.z), dimensions)
    }

    pub fn chunk_position(&self, dimensions: UVec2) -> IVec3 { self.position(dimensions).0 }

    pub fn local_position(&self, dimensions: UVec2) -> UVec2 { self.position(dimensions).1 }
}

// TryPosition
//######################
impl ChunkPosition {
    pub fn try_position(&self) -> Option<(IVec3, UVec2)> {
        if let DimensionType::Dimensions(v) = self.dimensions { Some(self.position(v)) } else { None }
    }

    pub fn try_chunk_position(&self) -> Option<IVec3> { self.try_position().map(|position| position.0) }

    pub fn try_local_position(&self) -> Option<UVec2> { self.try_position().map(|position| position.1) }
}

// Distance
//######################
impl ChunkPosition {
    pub fn distance(&self, other: Self) -> Option<u32> {
        if let Some((x, y)) = self.distance_both(other) { Some(x.max(y)) } else { None }
    }

    pub fn distance_x(&self, other: Self) -> Option<u32> {
        if let Some((x, _)) = self.distance_both(other) { Some(x) } else { None }
    }

    pub fn distance_y(&self, other: Self) -> Option<u32> {
        if let Some((_, y)) = self.distance_both(other) { Some(y) } else { None }
    }

    pub fn distance_both(&self, other: Self) -> Option<(u32, u32)> {
        if let Some((x1, y1, _z1)) = self.try_absolute() {
            if let Some((x2, y2, _z2)) = other.try_absolute() {
                return Some(((x2 - x1) as u32, (y2 - y1) as u32));
            }
        }
        None
    }
}

// Getters
//######################
impl ChunkPosition {
    pub fn z(&self) -> i32 { self.z }

    pub fn is_dimension(&self) -> bool { matches!(self.dimensions, DimensionType::Dimensions(_)) }

    pub fn dimensions(&self) -> Option<UVec2> {
        if let DimensionType::Dimensions(v) = self.dimensions { Some(v) } else { None }
    }

    /// Check dimensions first!!!
    pub fn as_absolute(&self) -> (i64, i64, i32) { (self.x, self.y, self.z) }
}

// Setters
//######################
impl ChunkPosition {
    pub fn set_dimensions(&mut self, dimensions: UVec2) {
        if let DimensionType::Modifier(m) = self.dimensions {
            self.x += m.0 * dimensions.x as i64;
            self.y += m.1 * dimensions.y as i64;
        }

        self.dimensions = DimensionType::Dimensions(dimensions);
    }
}

// Octant
//######################
impl ChunkPosition {
    pub fn octant_to(&self, other: Self) -> Option<Octant> {
        // adapted from <http://codereview.stackexchange.com/a/95551>

        if let DimensionType::Dimensions(d1) = self.dimensions {
            if let DimensionType::Dimensions(d2) = other.dimensions {
                if d1 == d2 {
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

                    return Some(Octant(octant));
                }
            }
        }
        None
    }
}

// Lerp
//######################
impl ChunkPosition {
    pub fn lerp(&self, other: Self, percent: f32) -> Self {
        let (abs_self_x, abs_self_y, abs_self_z) = self.as_absolute();
        let (abs_other_x, abs_other_y, abs_other_z) = other.as_absolute();

        let lerp_x = ((abs_other_x - abs_self_x) as f64).mul_add(percent as f64, abs_self_x as f64) as i64;
        let lerp_y = ((abs_other_y - abs_self_y) as f64).mul_add(percent as f64, abs_self_y as f64) as i64;
        let lerp_z = ((abs_other_z - abs_self_z) as f64).mul_add(percent as f64, abs_self_z as f64) as i32;

        let mut s = Self::from_raw(lerp_x, lerp_y, lerp_z);
        if let DimensionType::Dimensions(v) = self.dimensions {
            s.set_dimensions(v);
        }
        s
    }
}

// Private
//######################
impl ChunkPosition {
    fn from_absolute_position(absolute_position: (i64, i64, i32), dimensions: UVec2) -> (IVec3, UVec2) {
        let (world_x, local_x) = if absolute_position.0 < 0 {
            let abs_x = absolute_position.0.abs();
            let mut world = abs_x / dimensions.x as i64;
            let mut local = dimensions.x as i64 - (abs_x - (world * dimensions.x as i64));
            if local == dimensions.x as i64 {
                world -= 1;
                local = 0;
            }
            (-(world as i32) - 1, local as u32)
        } else {
            (
                (absolute_position.0 / dimensions.x as i64) as i32,
                (absolute_position.0 % dimensions.x as i64) as u32,
            )
        };

        let (world_y, local_y) = if absolute_position.1 < 0 {
            let abs_y = absolute_position.1.abs();
            let mut world = abs_y / dimensions.y as i64;
            let mut local = dimensions.y as i64 - (abs_y - (world * dimensions.y as i64));
            if local == dimensions.y as i64 {
                world -= 1;
                local = 0;
            }
            (-(world as i32) - 1, local as u32)
        } else {
            (
                (absolute_position.1 / dimensions.y as i64) as i32,
                (absolute_position.1 % dimensions.y as i64) as u32,
            )
        };

        (
            IVec3::new(world_x, world_y, absolute_position.2),
            UVec2::new(local_x, local_y),
        )
    }

    fn try_absolute(&self) -> Option<(i64, i64, i32)> {
        match self.dimensions {
            DimensionType::Dimensions(_d) => Some((self.x, self.y, self.z)),
            DimensionType::Modifier(m) => {
                if m.0 == 0 && m.1 == 0 {
                    Some((self.x, self.y, self.z))
                } else {
                    None
                }
            },
        }
    }
}

impl Default for ChunkPosition {
    fn default() -> Self { Self::new(IVec3::new(0, 0, 0), UVec2::new(0, 0)) }
}

impl PartialEq for ChunkPosition {
    fn eq(&self, other: &Self) -> bool {
        self.dimensions == other.dimensions && self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for ChunkPosition {}

impl Hash for ChunkPosition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.dimensions.hash(state);
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

impl Debug for ChunkPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self.dimensions {
            DimensionType::Dimensions(v) => {
                let position = self.position(v);
                format!(
                    "(x:{}, y:{}, z:{}::x:{}, y:{})",
                    position.0.x, position.0.y, position.0.z, position.1.x, position.1.y
                )
            },
            DimensionType::Modifier(m) => format!(
                "([mod_x:{}, mod_y:{}] x:{}, y:{}, z:{})",
                m.0, m.1, self.x, self.y, self.z
            ),
        };
        write!(f, "{text}")
    }
}

impl Display for ChunkPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self.dimensions {
            DimensionType::Dimensions(v) => {
                let position = self.position(v);
                format!(
                    "ChunkPosition {{\n\twidth: {}\n\theight: {}\n\n\tchunk_x: {},\n\tchunk_y: \
                     {},\n\tchunk_z: {}\n\n\tlocal_x: {},\n\tlocal_y: {},\n}}",
                    v.x, v.y, position.0.x, position.0.y, position.0.z, position.1.x, position.1.y
                )
            },
            DimensionType::Modifier(m) => format!(
                "ChunkPosition {{\n\tmod_x: {},\n\tmod_y: {},\n\n\tx: {},\n\ty: {},\n\tz: {}\n}}",
                m.0, m.1, self.x, self.y, self.z
            ),
        };
        write!(f, "{text}")
    }
}

impl Add<ChunkPosition> for ChunkPosition {
    type Output = ChunkPosition;

    fn add(self, rhs: ChunkPosition) -> Self::Output {
        match self.dimensions {
            DimensionType::Dimensions(dimensions) => {
                match rhs.dimensions {
                    DimensionType::Dimensions(v) => {
                        if dimensions == v {
                            Self {
                                dimensions: DimensionType::Dimensions(dimensions),
                                x: self.x + rhs.x,
                                y: self.y + rhs.y,
                                z: self.z + rhs.z,
                            }
                        } else {
                            // Which dimensions do we keep??? switch to lhs.dimensions??
                            // Keep none for now..
                            warn!("Add `ChunkPosition`: dimensions do not match!");

                            Self {
                                dimensions: DimensionType::Modifier((0, 0)),
                                x: self.x + rhs.x,
                                y: self.y + rhs.y,
                                z: self.z + rhs.z,
                            }
                        }
                    },
                    DimensionType::Modifier(_m) => {
                        let mut rhs = rhs;
                        rhs.set_dimensions(dimensions);
                        self + rhs
                    },
                }
            },
            DimensionType::Modifier(modifier) => match rhs.dimensions {
                DimensionType::Dimensions(dimensions) => {
                    let mut s = self;
                    s.set_dimensions(dimensions);
                    s + rhs
                },
                DimensionType::Modifier(m) => Self {
                    dimensions: DimensionType::Modifier((modifier.0 + m.0, modifier.1 + m.1)),
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: self.z + rhs.z,
                },
            },
        }
    }
}

impl Add<IVec2> for ChunkPosition {
    type Output = ChunkPosition;

    fn add(self, rhs: IVec2) -> Self::Output {
        Self {
            dimensions: self.dimensions,
            x: self.x + rhs.x as i64,
            y: self.y + rhs.y as i64,
            z: self.z,
        }
    }
}

impl AddAssign<IVec2> for ChunkPosition {
    fn add_assign(&mut self, rhs: IVec2) {
        self.x += rhs.x as i64;
        self.y += rhs.y as i64;
    }
}

impl Add<UVec2> for ChunkPosition {
    type Output = ChunkPosition;

    fn add(self, rhs: UVec2) -> Self::Output {
        Self {
            dimensions: self.dimensions,
            x: self.x + rhs.x as i64,
            y: self.y + rhs.y as i64,
            z: self.z,
        }
    }
}

impl AddAssign<UVec2> for ChunkPosition {
    fn add_assign(&mut self, rhs: UVec2) {
        self.x += rhs.x as i64;
        self.y += rhs.y as i64;
    }
}

impl Sub<ChunkPosition> for ChunkPosition {
    type Output = ChunkPosition;

    fn sub(self, rhs: ChunkPosition) -> Self::Output {
        match self.dimensions {
            DimensionType::Dimensions(dimensions) => {
                match rhs.dimensions {
                    DimensionType::Dimensions(v) => {
                        if dimensions == v {
                            Self {
                                dimensions: DimensionType::Dimensions(dimensions),
                                x: self.x - rhs.x,
                                y: self.y - rhs.y,
                                z: self.z - rhs.z,
                            }
                        } else {
                            // Which dimensions do we keep??? switch to lhs.dimensions??
                            // Keep none for now..
                            warn!("Sub `ChunkPosition`: dimensions do not match!");

                            Self {
                                dimensions: DimensionType::Modifier((0, 0)),
                                x: self.x - rhs.x,
                                y: self.y - rhs.y,
                                z: self.z - rhs.z,
                            }
                        }
                    },
                    DimensionType::Modifier(_m) => {
                        let mut rhs = rhs;
                        rhs.set_dimensions(dimensions);
                        self - rhs
                    },
                }
            },
            DimensionType::Modifier(modifier) => match rhs.dimensions {
                DimensionType::Dimensions(dimensions) => {
                    let mut s = self;
                    s.set_dimensions(dimensions);
                    s - rhs
                },
                DimensionType::Modifier(m) => Self {
                    dimensions: DimensionType::Modifier((modifier.0 + m.0, modifier.1 + m.1)),
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                    z: self.z - rhs.z,
                },
            },
        }
    }
}

impl Sub<IVec2> for ChunkPosition {
    type Output = ChunkPosition;

    fn sub(self, rhs: IVec2) -> Self::Output {
        Self {
            dimensions: self.dimensions,
            x: self.x - rhs.x as i64,
            y: self.y - rhs.y as i64,
            z: self.z,
        }
    }
}

impl SubAssign<IVec2> for ChunkPosition {
    fn sub_assign(&mut self, rhs: IVec2) {
        self.x -= rhs.x as i64;
        self.y -= rhs.y as i64;
    }
}

impl Sub<UVec2> for ChunkPosition {
    type Output = ChunkPosition;

    fn sub(self, rhs: UVec2) -> Self::Output {
        Self {
            dimensions: self.dimensions,
            x: self.x - rhs.x as i64,
            y: self.y - rhs.y as i64,
            z: self.z,
        }
    }
}

impl SubAssign<UVec2> for ChunkPosition {
    fn sub_assign(&mut self, rhs: UVec2) {
        self.x -= rhs.x as i64;
        self.y -= rhs.y as i64;
    }
}
