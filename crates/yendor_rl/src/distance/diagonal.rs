use std::{f32::consts::SQRT_2, ops::Sub};

use crate::prelude::*;

const CARDINAL_COST: f32 = 1.0;

/// The diagonal cost.
pub const DIAGONAL_COST: f32 = SQRT_2;

/// Calculates a Chebyshev distance between two points.
pub struct Diagonal;
impl DistanceAlgorithm for Diagonal {
    // Calculates a diagonal distance between two 2D points.
    fn distance2d(self, start: IVec2, end: IVec2) -> f32 {
        DiagonalWithCosts(CARDINAL_COST, DIAGONAL_COST).distance2d(start, end)
    }
}

/// Calculates a diagonal distance between two points.
pub struct DiagonalWithCosts(pub f32, pub f32);
impl DistanceAlgorithm for DiagonalWithCosts {
    // Calculates a diagonal distance with cost between two 2D points.
    fn distance2d(self, start: IVec2, end: IVec2) -> f32 {
        let start = start.as_vec2();
        let end = end.as_vec2();
        let distance = start.sub(end).abs();
        self.0.mul_add(
            distance.max_element(),
            (self.1 - self.0) * distance.min_element(),
        )
    }
}
