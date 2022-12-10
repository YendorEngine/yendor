use crate::prelude::*;
pub struct Row {
    depth: u32,
    start_slope: Slope,
    end_slope: Slope,
}
impl Row {
    pub const fn new(depth: u32, start_slope: Slope, end_slope: Slope) -> Self {
        Self {
            depth,
            start_slope,
            end_slope,
        }
    }

    pub const fn next(&self) -> Self {
        Self {
            depth: self.depth + 1,
            start_slope: self.start_slope,
            end_slope: self.end_slope,
        }
    }

    pub fn calc_starting_slope(&mut self, tile: IVec2) { self.start_slope = Self::slope(tile) }

    pub fn calc_ending_slope(&mut self, tile: IVec2) { self.end_slope = Self::slope(tile) }

    const fn slope(tile: IVec2) -> Slope { Slope::new(2 * tile.y - 1, 2 * tile.x) }

    pub fn tiles(&self) -> RowIter {
        RowIter {
            depth: self.depth,
            current_col: (self.depth as f64).mul_add(self.start_slope.value(), 0.5).floor() as i32, /* round up */
            max_col: (self.depth as f64).mul_add(self.end_slope.value(), -0.5).ceil() as i32, // round down
        }
    }

    pub fn is_symmetric(&self, tile: IVec2) -> bool {
        (tile.y as f64 >= self.depth as f64 * self.start_slope.value()) &
            (tile.y as f64 <= self.depth as f64 * self.end_slope.value())
    }
}
pub struct RowIter {
    depth: u32,
    max_col: i32,
    current_col: i32,
}
impl Iterator for RowIter {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current_col;
        self.current_col += 1;
        if current > self.max_col {
            None
        } else {
            Some(IVec2::new(self.depth as i32, current))
        }
    }
}
