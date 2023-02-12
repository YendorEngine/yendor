//! Opinionated game meta-engine built on Bevy.

#[doc(inline)]
pub use yendor_bevy_utils as bevy_utils;

/// Yendor lib prelude
pub mod prelude {
    pub use crate::bevy_utils::*;
}

/// This crate provides 2D camera shake using the methodology described in this excellent [GDC
/// talk](https://www.youtube.com/watch?v=tu-Qe66AvtY) by Squirrel Eiserloh.
#[cfg(feature = "camera_shake")]
pub mod camera_shake {
    pub use yendor_camera_shake::*;
}

#[cfg(feature = "roguelike")]
pub mod roguelike {
    pub use yendor_rl::*;
}
