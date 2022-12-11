use crate::prelude::*;

/// Calculates a Pythagoras distance between two points.
pub struct Pythagoras;

impl DistanceAlgorithm for Pythagoras {
    // Calculates a Pythagoras distance between two 2D points.
    fn distance2d(self, start: impl Point, end: impl Point) -> f32 {
        let distance_squared = PythagorasSquared.distance2d(start, end);
        f32::sqrt(distance_squared)
    }

    // Calculates a Pythagoras distance between two 3D points.
    fn distance3d(self, start: impl Point, end: impl Point) -> f32 {
        let distance_squared = PythagorasSquared.distance3d(start, end);
        f32::sqrt(distance_squared)
    }
}

/// Calculates a Pythagoras distance squared between two points.
pub struct PythagorasSquared;

impl DistanceAlgorithm for PythagorasSquared {
    fn distance2d(self, start: impl Point, end: impl Point) -> f32 {
        let start = start.as_vec2();
        let end = end.as_vec2();
        let distance = (start.max(end) - start.min(end)).powf(2.0);
        distance.x + distance.y
    }

    fn distance3d(self, _start: impl Point, _end: impl Point) -> f32 {
        unimplemented!("PythagorasSquared distance3d not implemented")
    }
}
