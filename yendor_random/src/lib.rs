#![allow(clippy::module_inception)]

mod random;
mod random_value;
pub use random::*;

mod imports;

pub mod prelude {
    pub(crate) use crate::imports::*;
    pub use crate::{random::*, random_value::*};
}
