use crate::prelude::*;

/// Calculates a Manhattan distance between two points
pub struct Manhattan;

impl DistanceAlgorithm for Manhattan {
    /// Calculates a Manhattan distance between two 2D points
    fn distance2d(self, start: IVec2, end: IVec2) -> f32 {
        let start = start.as_vec2();
        let end = end.as_vec2();
        let distance = start.max(end) - start.min(end);
        distance.x + distance.y
    }
}
