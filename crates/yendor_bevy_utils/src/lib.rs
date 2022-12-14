//! Bevy Helper Utilities

#![warn(missing_docs)]
// This cfg_attr is needed because `rustdoc::all` includes lints not supported on stable
#![cfg_attr(doc, allow(unknown_lints))]
#![deny(rustdoc::all)]

mod macros;
pub use macros::*;

/// The prelude.
pub mod prelude {
    pub use crate::{switch_app_state, *};
}

/// Helper trait for converting yendor types to Bevy types.
pub trait IntoBevy<To> {
    /// Convert the type to a Bevy type.
    fn into_bevy(self) -> To;
}
