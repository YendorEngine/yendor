#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Bevy Helper Utilities

#![warn(missing_docs)]
// This cfg_attr is needed because `rustdoc::all` includes lints not supported on stable
#![cfg_attr(doc, allow(unknown_lints))]

/// The prelude.
pub mod prelude {
    pub use crate::*;
}

/// Helper trait for converting to Bevy types.
pub trait IntoBevy<To> {
    /// Convert the type to a Bevy type.
    fn into_bevy(self) -> To;
}
