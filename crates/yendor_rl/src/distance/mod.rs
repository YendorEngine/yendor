use crate::prelude::*;

mod chebyshev;
pub use chebyshev::*;
mod diagonal;
pub use diagonal::*;
mod distance_algorithm;
pub use distance_algorithm::*;
mod manhattan;
pub use manhattan::*;
mod pythagoras;
pub use pythagoras::*;

/// Enumeration of available 2D/3D Distance algorithms
pub enum Distance {
    /// Use the Pythagoras algorithm for determining distance - sqrt(A^2 + B^2)
    Pythagoras,
    /// Us the Pythagoras algorithm for distance, but omitting the square-root for a faster
    /// but squared result.
    PythagorasSquared,
    /// Use Manhattan distance (distance up plus distance along)
    Manhattan,
    /// Use Chebyshev distance (like Manhattan, but adds one to each entry)
    Chebyshev,
    /// Use a diagonal distance, the max of the x and y distances
    Diagonal,
    /// Use a diagonal distance, the max of the x and y distances
    DiagonalWithCosts(f32, f32),
}

impl Distance {
    /// Provides a 2D distance between points, using the specified algorithm.
    pub fn distance2d(self, start: IVec2, end: IVec2) -> f32 {
        match self {
            Self::Manhattan => Manhattan.distance2d(start, end),
            Self::Chebyshev => Chebyshev.distance2d(start, end),
            Self::Diagonal => Diagonal.distance2d(start, end),
            Self::Pythagoras => Pythagoras.distance2d(start, end),
            Self::PythagorasSquared => PythagorasSquared.distance2d(start, end),
            Self::DiagonalWithCosts(d1, d2) => DiagonalWithCosts(d1, d2).distance2d(start, end),
        }
    }
}
