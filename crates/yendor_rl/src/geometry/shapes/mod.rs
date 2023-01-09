mod iter {
    mod rect_iter;
    pub use rect_iter::*;
    mod line_iter;
    pub use line_iter::*;
}
pub use iter::*;

mod octant;
pub use octant::*;
mod circle;
pub use circle::*;
mod line;
pub use line::*;
mod rectangle;
pub use rectangle::*;
mod shape;
pub use shape::*;
