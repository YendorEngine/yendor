//! ULID-related utilities such as ULID map and type ULIDs.
//!
//! - [`Ulid`] comes from the [`ulid`] crate.
//!
//! [`ulid`]: https://docs.rs/ulid

use fxhash::FxHashMap;
use ulid::Ulid;

/// Faster hash map using [`FxHashMap`] and a ULID key.
pub type UlidMap<T> = FxHashMap<Ulid, T>;
