use crate::prelude::*;

// TODO: Generic the HashTable
// TODO: Generic the Noise
pub struct Random<Rng: RngCore + SeedableRng> {
    prng: Prng<Rng>,
    prht: Prht,
    noise: Noise,
}

impl<Rng: RngCore + SeedableRng> Random<Rng> {
    pub fn new(seed: u64) -> Self {
        let mut prng = Prng::new(seed);
        let prht = Prht::new(prng.next_u64());
        let noise = Noise::new(prng.next());
        Self { prng, prht, noise }
    }

    pub fn from_entropy() -> Self { Self::new(Prng::<Rng>::entropy_u64()) }

    pub fn get_prng(&mut self) -> &mut Prng<Rng> { &mut self.prng }
    pub fn get_prht(&mut self) -> &mut Prht { &mut self.prht }
    pub fn get_noise(&mut self) -> &mut Noise { &mut self.noise }
}

impl<Rng: RngCore + SeedableRng> Default for Random<Rng> {
    fn default() -> Self { Self::from_entropy() }
}

impl<Rng: RngCore + SeedableRng> std::fmt::Debug for Random<Rng> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Random({:?})", self.prng.seed())
    }
}
