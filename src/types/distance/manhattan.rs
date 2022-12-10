// Include public
// Include Private
use super::*;
use crate::prelude::*;

/// Calculates a Manhattan distance between two points
pub struct Manhattan;

impl DistanceAlgorithm for Manhattan {
    /// Calculates a Manhattan distance between two 2D points
    fn distance2d(self, start: impl Point, end: impl Point) -> f32 {
        let start = start.as_vec2();
        let end = end.as_vec2();
        let distance = start.max(end) - start.min(end);
        distance.x + distance.y
    }

    /// Calculates a Manhattan distance between two 3D points
    fn distance3d(self, _start: impl Point, _end: impl Point) -> f32 {
        unimplemented!("Manhattan distance3d not implemented")
    }
}
