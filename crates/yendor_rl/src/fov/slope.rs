/// represents the slope Y/X as a rational number
#[derive(Clone, Copy)]
pub struct Slope {
    pub y: i32,
    pub x: i32,
}
impl Slope {
    #[inline]
    pub const fn new(y: i32, x: i32) -> Self { Self { y, x } }

    // f64 y / x
    #[inline]
    pub fn value(&self) -> f64 { self.y as f64 / self.x as f64 }

    // this > y/x
    #[inline]
    pub const fn greater(&self, y: i32, x: i32) -> bool { self.y * x > self.x * y }

    // s >= y/x
    #[inline]
    pub const fn greater_or_equal(&self, y: i32, x: i32) -> bool { self.y * x >= self.x * y }

    // s <= y/x
    #[inline]
    pub const fn less_or_equal(&self, y: i32, x: i32) -> bool { self.y * x <= self.x * y } // this <= y/x
}
