use std::fmt::{self, Debug};

use crate::prelude::*;
#[derive(Clone)]
pub struct Prht {
    seed: u64,
    xxhash: Xxh3,
}
#[allow(dead_code)]
impl Prht {
    pub const fn new(seed: u64) -> Self {
        let xxhash = Xxh3Builder::new().with_seed(seed).build();
        Self { seed, xxhash }
    }

    pub fn get<X: Into<i64>, Y: Into<i64>, Z: Into<i64>>(&mut self, x: X, y: Y, z: Z) -> u64 {
        let x = x.into();
        let y = y.into();
        let z = z.into();
        self.xxhash.reset();
        self.xxhash.update(&x.to_be_bytes());
        self.xxhash.update(&y.to_be_bytes());
        self.xxhash.update(&z.to_be_bytes());
        self.xxhash.digest()
    }
}
impl Serialize for Prht {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let mut state = serializer.serialize_struct("Prht", 1)?;
        state.serialize_field("seed", &self.seed)?;
        state.end()
    }
}
impl<'de> Deserialize<'de> for Prht {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where D: Deserializer<'de> {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Seed,
        }
        // impl<'de> Deserialize<'de> for Field {
        // fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        // struct FieldVisitor;
        //
        // impl<'de> Visitor for FieldVisitor {
        // fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        // formatter.write_str("'seed'")
        // }
        //
        // fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: de::Error, {
        // match v {
        // "seed" => Ok(Field::Seed)
        // _ => Err(de::Error::unknown_field(v, FIELDS))
        // }
        // }
        // }
        // }
        // }
        struct PrhtVisitor;
        impl<'de> Visitor<'de> for PrhtVisitor {
            type Value = Prht;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Prht")
            }

            fn visit_seq<A>(self, mut seq: A) -> core::result::Result<Self::Value, A::Error>
            where A: SeqAccess<'de> {
                let seed = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                Ok(Prht::new(seed))
            }

            fn visit_map<A>(self, mut map: A) -> core::result::Result<Self::Value, A::Error>
            where A: MapAccess<'de> {
                let mut seed = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Seed => {
                            if seed.is_some() {
                                return Err(de::Error::duplicate_field("seed"));
                            }
                            seed = Some(map.next_value()?);
                        },
                    }
                }
                let seed = seed.ok_or_else(|| de::Error::missing_field("seed"))?;
                Ok(Prht::new(seed))
            }
        }
        const FIELDS: &[&str] = &["seed"];
        deserializer.deserialize_struct("Prht", FIELDS, PrhtVisitor)
    }
}
impl Debug for Prht {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Prht {{ seed:{} }}", self.seed))
    }
}
