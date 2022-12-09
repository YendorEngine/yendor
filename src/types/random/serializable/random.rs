use super::prng::SerializablePrng;
use crate::prelude::*;
// TODO: Generic the HashTable
// TODO: Generic the Noise
#[derive(Serialize, Deserialize, Clone)]
pub struct SerializableRandom<Rng: RngCore + SeedableRng + Serialize> {
    prng: SerializablePrng<Rng>,
    prht: Prht,
    noise: Noise,
}

impl<Rng: RngCore + SeedableRng + Serialize> SerializableRandom<Rng> {
    pub fn new(seed: u64) -> Self {
        let mut prng = SerializablePrng::new(seed);
        let prht = Prht::new(prng.next_u64());
        let noise = Noise::new(prng.next());
        Self { prng, prht, noise }
    }

    pub fn from_entropy() -> Self {
        Self::new(SerializablePrng::<Rng>::entropy_u64())
    }

    pub fn get_prng(&mut self) -> &mut SerializablePrng<Rng> {
        &mut self.prng
    }
    pub fn get_prht(&mut self) -> &mut Prht {
        &mut self.prht
    }
    pub fn get_noise(&mut self) -> &mut Noise {
        &mut self.noise
    }
}
impl<Rng: RngCore + SeedableRng + Serialize> Default for SerializableRandom<Rng> {
    fn default() -> Self {
        Self::from_entropy()
    }
}
impl<Rng: RngCore + SeedableRng + Serialize> std::fmt::Debug for SerializableRandom<Rng> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Random({:?})", self.prng.seed())
    }
}
