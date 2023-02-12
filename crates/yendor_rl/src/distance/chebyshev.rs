use std::ops::Sub;

use crate::prelude::*;

/// Calculates a Chebyshev distance between two points.
pub struct Chebyshev;

impl DistanceAlgorithm for Chebyshev {
    /// Calculates a Chebyshev distance between two 2D points
    /// See: [`GameProgramming` Heuristics](http://theory.stanford.edu/~amitp/GameProgramming/Heuristics.html)
    fn distance2d(self, start: IVec2, end: IVec2) -> f32 {
        let start = start.as_vec2();
        let end = end.as_vec2();
        start.sub(end).abs().max_element()
    }
}
