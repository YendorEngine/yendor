//! ULID-related utilities such as ULID map and type ULIDs.
//!
//! - [`TypeUlid`] comes from the [`yendor_type_ulid`] crate
//! - [`Ulid`] comes from the [`ulid`] crate.
//!
//! [`ulid`]: https://docs.rs/ulid

use fxhash::FxHashMap;
pub use yendor_type_ulid::{TypeUlid, Ulid};

/// Faster hash map using [`FxHashMap`] and a ULID key.
pub type UlidMap<T> = FxHashMap<Ulid, T>;
