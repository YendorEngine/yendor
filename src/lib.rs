// https://github.com/rust-lang/rust/issues/95174
#![feature(adt_const_params)]
// https://github.com/rust-lang/rust/issues/85077
#![feature(generic_arg_infer)]

pub mod types;
pub mod utilities;

mod imports;

pub mod prelude {
    pub(crate) use crate::imports::*;
    pub use crate::{types::*, utilities::*};
}
