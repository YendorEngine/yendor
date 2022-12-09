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
    layer: u32,
    delta_x: i64,
    delta_y: i64,
    octant: Octant,
}

impl BresenhamLineIter {
    /// Creates a new iterator.Yields intermediate points between `start`
    /// and `end`. Does include `start` but not `end`.
    #[inline]
    pub fn new(start: Position, end: Position) -> Self {
        // figure out which octant `end` is relative to `start`
        let octant = start.octant_to(end);

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
            abs_z: start.world_z(),
            layer: start.layer(),
            delta_step: delta_y - delta_x,
        }
    }

    /// Return the next point without checking if we are past `end`.
    #[inline]
    pub fn advance(&mut self) -> Position {
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
        self.octant.from_offset(current_point, self.abs_z, self.layer)
    }
}

impl Iterator for BresenhamLineIter {
    type Item = Position;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.abs_x >= self.end_x {
            None
        } else {
            Some(self.advance())
        }
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
    pub fn new(start: Position, end: Position) -> Self { Self(BresenhamLineIter::new(start, end)) }

    /// Return the next point without checking if we are past `end`.
    #[inline]
    pub fn advance(&mut self) -> Position { self.0.advance() }
}

impl Iterator for BresenhamLineInclusiveIter {
    type Item = Position;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.abs_x > self.0.end_x {
            None
        } else {
            Some(self.0.advance())
        }
    }
}

//#[cfg(test)]
// mod tests {
//    #[cfg(test)]
//    mod line {
//        use crate::prelude::*;
//
//        #[test]
//        fn line_vertical() {
//            let mut canvas = Canvas::new([11, 11]);
//            let line = Line::new((0, 0), (0, 10));
//            for p in line.iter() {
//                canvas.put(p, '*');
//            }
//            canvas.print();
//        }
//
//        #[test]
//        fn line_horizontal() {
//            let mut canvas = Canvas::new([11, 11]);
//            let line = Line::new((0, 0), (10, 0));
//            for p in line.iter() {
//                canvas.put(p, '*');
//            }
//            canvas.print();
//        }
//
//        #[test]
//        fn line_diagonal() {
//            let mut canvas = Canvas::new([10, 10]);
//            let line = Line::new((0, 0), (9, 9));
//            for p in line.iter() {
//                canvas.put(p, '*');
//            }
//            canvas.print();
//        }
//
//        #[test]
//        fn line_multi() {
//            let mut canvas = Canvas::new([11, 7]);
//            let lines = [
//                Line::new((5, 3), (10, 5)),
//                Line::new((5, 3), (10, 1)),
//                Line::new((5, 3), (0, 1)),
//                Line::new((5, 3), (0, 5)),
//            ];
//            for p in lines.iter().flat_map(|l| l.iter()) {
//                canvas.put(p, '*');
//            }
//            canvas.print();
//        }
//    }
//
//    #[cfg(test)]
//    mod bresenham {
//        use crate::prelude::*;
//
//        #[test]
//        fn line_diagonal() {
//            let mut canvas = Canvas::new([10, 10]);
//            let line = BresenhamLineIter::new((0, 0), (9, 9));
//            for p in line {
//                canvas.put(p, '*');
//            }
//            canvas.print();
//        }
//    }
//
//    #[cfg(test)]
//    mod bresenham_inclusive {
//        use crate::prelude::*;
//
//        #[test]
//        fn line_diagonal() {
//            let mut canvas = Canvas::new([10, 10]);
//            let line = BresenhamLineInclusiveIter::new((0, 0), (9, 9));
//            for p in line {
//                canvas.put(p, '*');
//            }
//            canvas.print();
//        }
//    }
//}
