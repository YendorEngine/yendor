pub use bevy::{
    math::{IVec2, IVec3, UVec2, UVec3, Vec2, Vec3},
    prelude::{Deref, DerefMut},
    utils::{HashMap, HashSet},
};

#[cfg(feature = "reflect")]
pub use bevy::reflect::{FromReflect, Reflect};

#[cfg(feature = "random")]
pub use ::noise::{NoiseFn, Perlin as PerlinNoise};

#[cfg(feature = "random")]
pub use rand::{
    distributions::{Distribution, Standard},
    prelude::*,
    Rng as RandRng, SeedableRng,
};
#[cfg(feature = "serialize")]
pub use serde::{
    de::{self, Deserializer, MapAccess, SeqAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Serialize,
};
#[cfg(feature = "random")]
pub use xxhash_rust::xxh3::{Xxh3, Xxh3Builder};
