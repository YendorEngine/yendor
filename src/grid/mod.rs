pub mod grids {
    #[cfg(feature = "reflect")]
    mod reflect {
        mod grid_2d;
        pub use grid_2d::*;
    }
    #[cfg(feature = "reflect")]
    pub use reflect::*;

    #[cfg(feature = "bitgrid")]
    mod bitgrid;
    #[cfg(feature = "bitgrid")]
    pub use bitgrid::*;
    mod grid_2d;
    pub use grid_2d::*;
    mod grid_3d;
    pub use grid_3d::*;
}
pub use grids::*;

pub mod point {
    mod convertable;
    pub use convertable::*;
    mod grid_point;
    pub use grid_point::*;
    mod point_iter;
    pub use point_iter::*;
}
pub use point::*;

pub mod shapes {
    mod iter {
        mod rect_iter;
        pub use rect_iter::*;
        mod line_iter;
        pub use line_iter::*;
    }
    pub(crate) use iter::*;

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
}
pub use shapes::*;

mod grid_iterable;
pub use grid_iterable::*;
mod grid_layer;
pub use grid_layer::*;
