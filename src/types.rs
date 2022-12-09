pub mod dimensions {
    mod dimensions;
    pub use dimensions::*;
}
pub use dimensions::*;

pub mod directions {
    mod cardinal_direction;
    pub use cardinal_direction::*;
    mod direction_flags;
    pub(crate) use direction_flags::*;
    mod direction_iter;
    pub(crate) use direction_iter::*;
    mod direction_type;
    pub use direction_type::*;
    mod direction;
    pub use direction::*;
    mod ordinal_direction;
    pub use ordinal_direction::*;
    mod vertical_direction;
    pub use vertical_direction::*;
}
pub use directions::*;

pub mod point {
    mod point;
    pub use point::*;
    mod point_iter;
    pub(crate) use point_iter::*;
}
pub use point::*;

pub mod position {
    mod local_position;
    pub use local_position::*;
    mod octant;
    pub use octant::*;
    mod position;
    pub use position::*;
    mod world_position;
    pub use world_position::*;
}
pub use position::*;

pub mod shapes {
    mod iter {
        mod rect_iter;
        pub use rect_iter::*;
        mod line_iter;
        pub use line_iter::*;
    }
    pub(crate) use iter::*;

    mod circle;
    pub use circle::*;
    mod line;
    pub use line::*;
    mod rectangle;
    pub use rectangle::*;

    mod shape;
    pub use shape::*;
}