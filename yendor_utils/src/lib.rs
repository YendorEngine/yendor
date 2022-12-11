#![allow(clippy::module_inception)]

mod range;

pub mod prelude {
    pub use crate::range::*;
}
