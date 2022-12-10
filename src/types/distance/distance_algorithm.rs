// Include public
use crate::prelude::*;

pub trait DistanceAlgorithm {
    /// Provides a 2D distance between points, using the specified algorithm.
    fn distance2d(self, start: impl Point, end: impl Point) -> f32;

    /// Provides a 3D distance between points, using the specified algorithm.
    fn distance3d(self, start: impl Point, end: impl Point) -> f32;
}

