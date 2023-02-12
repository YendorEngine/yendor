/// represents the slope Y/X as a rational number
#[derive(Clone, Copy)]
pub struct Slope {
    /// y value
    pub y: i32,
    /// x value
    pub x: i32,
}
impl Slope {
    /// creates a new slope
    #[inline]
    pub const fn new(y: i32, x: i32) -> Self {
        Self { y, x }
    }

    /// y/x
    #[inline]
    pub fn value(&self) -> f64 {
        self.y as f64 / self.x as f64
    }

    /// slope > y/x
    #[inline]
    pub const fn greater(&self, y: i32, x: i32) -> bool {
        self.y * x > self.x * y
    }

    /// slope >= y/x
    #[inline]
    pub const fn greater_or_equal(&self, y: i32, x: i32) -> bool {
        self.y * x >= self.x * y
    }

    /// slope <= y/x
    #[inline]
    pub const fn less_or_equal(&self, y: i32, x: i32) -> bool {
        self.y * x <= self.x * y
    }
}
