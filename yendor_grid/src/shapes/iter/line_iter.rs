use crate::prelude::*;

//////////////////////////////////////////////////////////////////////////////////////////
/// Bresenham Algo
//////////////////////////////////////////////////////////////////////////////////////////

/// Line-drawing iterator
#[derive(Debug, Clone)]
pub struct BresenhamLineIter {
    abs_x: i64,      // Absolute start.x
    abs_y: i64,      // Absolute start.y
    abs_z: i32,      // Absolute start.z
    end_x: i64,      // Offset end.x
    delta_step: i64, // number of steps before we need to change y
    delta_x: i64,
    delta_y: i64,
    octant: Octant,
}

impl BresenhamLineIter {
    /// Creates a new iterator.Yields intermediate points between `start`
    /// and `end`. Does include `start` but not `end`.
    #[inline]
    pub fn new(start: ChunkPosition, end: ChunkPosition) -> Self {
        // figure out which octant `end` is relative to `start`
        let octant = start.octant_to(end).expect("Error making line.");

        let start_offset = octant.to_offset(start);
        // convert end to a relative offset in `Octant(0)`
        let end_offset = octant.to_offset(end);

        // distance along `X` axis
        let delta_x = end_offset.0 - start_offset.0;
        // distance along `Y` axis
        let delta_y = end_offset.1 - start_offset.1;

        Self {
            octant,
            delta_x,
            delta_y,
            end_x: end_offset.0,
            abs_x: start_offset.0,
            abs_y: start_offset.1,
            abs_z: start.z(),
            delta_step: delta_y - delta_x,
        }
    }

    /// Return the next point without checking if we are past `end`.
    #[inline]
    pub fn advance(&mut self) -> ChunkPosition {
        let current_point = (self.abs_x, self.abs_y);
        if self.delta_step >= 0 {
            self.abs_y += 1; // we can add because self.end_x is to the right and up(Octant(0))
            self.delta_step -= self.delta_x;
        }

        self.delta_step += self.delta_y;

        // loop inc
        self.abs_x += 1; // we can add because self.end_x is to the right and up(Octant(0))
        // we are moving towards the `end` offset into `Octant()`, so now we must wrap this new
        // coordinate back around to the original direction.
        self.octant.from_offset(current_point, self.abs_z)
    }
}

impl Iterator for BresenhamLineIter {
    type Item = ChunkPosition;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.abs_x >= self.end_x { None } else { Some(self.advance()) }
    }
}
//////////////////////////////////////////////////////////////////////////////////////////
/// Bresenham inclusive Algo
//////////////////////////////////////////////////////////////////////////////////////////

/// New type over `Bresenham` which include the `end` points when iterated over.
#[derive(Debug, Clone)]
pub struct BresenhamLineInclusiveIter(BresenhamLineIter);

impl BresenhamLineInclusiveIter {
    /// Creates a new iterator. Yields points `start..=end`.
    #[inline]
    pub fn new(start: ChunkPosition, end: ChunkPosition) -> Self { Self(BresenhamLineIter::new(start, end)) }

    /// Return the next point without checking if we are past `end`.
    #[inline]
    pub fn advance(&mut self) -> ChunkPosition { self.0.advance() }
}

impl Iterator for BresenhamLineInclusiveIter {
    type Item = ChunkPosition;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.abs_x > self.0.end_x { None } else { Some(self.0.advance()) }
    }
}
