//! An asset integration between Bevy and bones.
//!
//! Provides an easy way to load metadata for bones games using Bevy assets.

#![warn(missing_docs)]
// This cfg_attr is needed because `rustdoc::all` includes lints not supported on stable
#![cfg_attr(doc, allow(unknown_lints))]
#![deny(rustdoc::all)]

use bevy::{
    app::App,
    asset as bevy_asset,
    prelude::{UVec2, Vec2, Vec3},
    reflect::TypeUuid,
};
use bevy_asset::Asset;

/// The prelude.
pub mod prelude {
    pub use yendor_lib::prelude as bones;
    pub use yendor_type_ulid::TypeUlid;

    pub use crate::*;
}

use prelude::*;
pub use yendor_bevy_asset_macros::{BonesBevyAsset, BonesBevyAssetLoad};

#[doc(hidden)]
pub mod _private {
    pub use serde_json;
    pub use serde_yaml;
}

/// Dummy asset needed as a type parameter for the `load_context.get_handle` method that
/// doesn't have an untyped equivalent.
#[derive(TypeUuid)]
#[uuid = "ece514f7-4ffe-4251-9c25-d568acd696eb"]
struct DummyAsset;

/// Trait that may be derived to implement a Bevy asset type.
// TODO: Integrate or move HasLoadProgress with BonesBevyAsset.
pub trait BonesBevyAsset: TypeUlid + Asset {
    /// Install the asset loader for this type.
    fn install_asset(app: &mut App);
}

/// Extension trait for [`App`] that makes it easy to register bones assets.
pub trait BonesBevyAssetAppExt {
    /// Adds a [`BonesBevyAsset`] to the app, including it's asset loader.
    fn add_yendor_asset<T: BonesBevyAsset>(&mut self) -> &mut Self;
}

impl BonesBevyAssetAppExt for App {
    fn add_yendor_asset<T: BonesBevyAsset>(&mut self) -> &mut Self {
        T::install_asset(self);

        self
    }
}

/// Trait implemented for types that may appear in the fields of a [`BonesBevyAsset`] and may
/// need to perform aditional loading with the bevy load context.
pub trait BonesBevyAssetLoad {
    /// Allows the field to do any extra loading that it might need to do from the Bevy load
    /// context when the asset is loaded.
    fn load(
        &mut self,
        load_context: &mut bevy_asset::LoadContext,
        dependencies: &mut Vec<bevy_asset::AssetPath<'static>>,
    ) {
        let _ = (load_context, dependencies);
    }
}

impl<T: TypeUlid> BonesBevyAssetLoad for bones::Handle<T> {
    fn load(
        &mut self,
        load_context: &mut bevy_asset::LoadContext,
        dependencies: &mut Vec<bevy_asset::AssetPath<'static>>,
    ) {
        // Convert this path to a path relative to the parent asset
        self.path.normalize_relative_to(load_context.path());

        // Create a bevy asset path from this bones handle
        let asset_path = bevy_asset::AssetPath::new(
            self.path.path.to_path_buf(),
            self.path.clone().label.map(|x| x.to_string()),
        );
        let path_id = asset_path.get_id();
        dependencies.push(asset_path);

        // Load the asset
        let handle = load_context.get_handle::<_, DummyAsset>(path_id);

        // Leak the strong handle so that the asset doesn't get unloaded
        std::mem::forget(handle);
    }
}

impl<T: BonesBevyAssetLoad> BonesBevyAssetLoad for Vec<T> {
    fn load(
        &mut self,
        load_context: &mut bevy_asset::LoadContext,
        dependencies: &mut Vec<bevy_asset::AssetPath<'static>>,
    ) {
        self.iter_mut().for_each(|x| x.load(load_context, dependencies))
    }
}

/// Helper make empty load implementations for a list of types.
macro_rules! impl_default_traits {
    ( $($type:ty),* $(,)? ) => {
        $(
            impl BonesBevyAssetLoad for $type {}
        )*
    };
}

// Implement for types that don't need special loading behavior.
impl_default_traits!(
    String, f32, f64, usize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, Vec2, Vec3, UVec2, bool
);
