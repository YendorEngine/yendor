// Include public
use crate::prelude::*;

pub trait DistanceAlgorithm {
    /// Provides a 2D distance between points, using the specified algorithm.
    fn distance2d(self, start: IVec2, end: IVec2) -> f32;
}
