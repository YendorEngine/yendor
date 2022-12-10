#![allow(clippy::module_inception)]

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
    mod direction_table;
    pub(crate) use direction_table::*;
    mod direction_type;
    pub(crate) use direction_type::*;
    mod direction;
    pub use direction::*;
    mod ordinal_direction;
    pub use ordinal_direction::*;
    mod vertical_direction;
    pub use vertical_direction::*;
}
pub use directions::*;

mod distance {
    mod distance_algorithm;
    pub use distance_algorithm::*;
    mod chebyshev;
    pub use chebyshev::*;
    mod diagonal;
    pub use diagonal::*;
    mod manhattan;
    pub use manhattan::*;
    mod pythagoras;
    pub use pythagoras::*;
}
pub use distance::*;

pub mod point {
    mod point_impl;
    pub use point_impl::*;
    mod point_iter;
    pub(crate) use point_iter::*;
    mod point;
    pub use point::*;
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

#[cfg(feature = "random")]
pub mod random {
    mod noise;
    pub use self::noise::*;
    mod prht;
    pub use prht::*;
    mod prng;
    pub use prng::*;
    mod random;
    pub use random::*;

    #[cfg(feature = "serialize")]
    pub mod serializable {
        mod prng;
        pub use prng::*;
        mod random;
        pub use random::*;
    }
    #[cfg(feature = "serialize")]
    pub use serializable::*;
}
#[cfg(feature = "random")]
pub use random::*;

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
