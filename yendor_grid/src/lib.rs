#![feature(trait_alias)]
#![allow(incomplete_features)]
#![allow(clippy::module_inception)]
// https://github.com/rust-lang/rust/issues/95174
#![feature(adt_const_params)]
// https://github.com/rust-lang/rust/issues/85077
#![feature(generic_arg_infer)]


pub mod chunked_grid {
    mod chunk_position;
    pub use chunk_position::*;
    mod chunked_grid;
    pub use chunked_grid::*;
    mod octant;
    pub use octant::*;
}

pub mod dimensions {
    mod dimensions;
    pub use dimensions::*;
}

pub mod directions {
    mod cardinal_direction;
    pub use cardinal_direction::*;
    mod direction_flags;
    pub(crate) use direction_flags::*;
    mod direction_iter;
    pub(crate) use direction_iter::*;
    mod direction_iterator;
    pub use direction_iterator::*;
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

pub mod grids {
    #[cfg(feature = "bitvec")]
    mod bitgrid;
    #[cfg(feature = "bitvec")]
    pub use bitgrid::*;
    mod grid_2d;
    pub use grid_2d::*;
    mod grid_3d;
    pub use grid_3d::*;
}

#[cfg(feature = "reflect")]
mod reflect {
    mod grid_2d;
    pub use grid_2d::*;
}

pub mod point {
    mod point_impl;
    pub use point_impl::*;
    mod point_iter;
    pub use point_iter::*;
    mod point;
    pub use point::*;
}

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


mod grid_iterable;
mod grid_layer;

pub mod imports;

pub mod prelude {
    pub(crate) use crate::imports::*;
    pub use crate::{
        chunked_grid::*,
        dimensions::*,
        directions::*,
        grids::*,
        point::*,
        shapes::*,
        grid_iterable::*,
        grid_layer::*,
        reflect::*,
    };
}
