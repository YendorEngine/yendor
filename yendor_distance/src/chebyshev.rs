use std::ops::Sub;

use crate::prelude::*;

/// Calculates a Chebyshev distance between two points.
pub struct Chebyshev;

impl DistanceAlgorithm for Chebyshev {
    /// Calculates a Chebyshev distance between two 2D points
    /// See: http://theory.stanford.edu/~amitp/GameProgramming/Heuristics.html
    fn distance2d(self, start: impl Point, end: impl Point) -> f32 {
        let start = start.as_vec2();
        let end = end.as_vec2();
        start.sub(end).abs().max_element()
    }

    /// Calculates a Chebyshev distance between two 3D points
    fn distance3d(self, _start: impl Point, _end: impl Point) -> f32 {
        unimplemented!("Chebyshev distance3d not implemented")
    }
}
