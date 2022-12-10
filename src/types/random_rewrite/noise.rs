use core::hash::Hasher;

use crate::prelude::*;



pub enum Noise {
    Perlin,
}

impl Noise {
    pub fn new(self, seed: u64) -> Box<dyn Hasher> {
        match self {
            Self::Perlin => Box::new(::noise::Perlin::new(seed as u32))
        }
    }
}