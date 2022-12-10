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
}
