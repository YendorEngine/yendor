use std::ops::RangeBounds;

use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct SerializablePrng<Rng: RngCore + SeedableRng + Serialize> {
    seed: u64,
    rng: Rng,
}

#[allow(dead_code)]
impl<Rng: RngCore + SeedableRng + Serialize> SerializablePrng<Rng> {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            rng: Rng::seed_from_u64(seed),
        }
    }

    pub fn from_entropy() -> Self {
        Self::new(Rng::from_entropy().next_u64())
    }

    pub fn as_rng(&mut self) -> &mut Rng {
        &mut self.rng
    }

    pub const fn seed(&self) -> u64 {
        self.seed
    }

    pub fn coin(&mut self) -> bool {
        self.max_inclusive(1) == 1
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> u32 {
        self.rng.next_u32()
    }

    pub fn max(&mut self, max: u32) -> u32 {
        match max {
            0 => max,
            _ => self.max_inclusive(max - 1),
        }
    }

    pub fn max_inclusive(&mut self, max: u32) -> u32 {
        if max == 0 {
            return max;
        }
        let top = max as u64 + 1;
        let buckets = (u32::MAX as u64 / top).max(1);
        let limit = (buckets * top).max(1);
        let mut x;
        loop {
            x = self.next() as u64;
            if x < limit {
                break;
            }
        }
        (x / buckets) as u32
    }

    pub fn range<R: RangeBounds<u32>>(&mut self, range: R) -> u32 {
        let (start, end) = get_range_bounds(range, u32::MIN, u32::MAX);
        self.max_inclusive(end - start) + start
    }

    pub fn max_u64(&mut self, max: u64) -> u64 {
        match max {
            0 => max,
            _ => self.max_inclusive_u64(max - 1),
        }
    }

    pub fn max_inclusive_u64(&mut self, max: u64) -> u64 {
        if max == 0 {
            return max;
        }
        let top = max as u128 + 1;
        let buckets = (u64::MAX as u128 / top).max(1);
        let limit = (buckets * top).max(1);
        let mut x;
        loop {
            x = self.next_u64() as u128;
            if x < limit {
                break;
            }
        }
        (x / buckets) as u64
    }

    pub fn range_u64<R: RangeBounds<u64>>(&mut self, range: R) -> u64 {
        let (start, end) = get_range_bounds(range, u64::MIN, u64::MAX);
        self.max_inclusive_u64(end - start) + start
    }

    pub fn next_usize(&mut self) -> usize {
        self.rng.gen::<usize>()
    }

    pub fn next_i32(&mut self) -> i32 {
        self.rng.gen::<i32>()
    }

    pub fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }

    pub fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    pub fn next_f32(&mut self) -> f32 {
        (self.next() as f64 / (u32::MAX as u64 + 1) as f64) as f32
    }

    pub fn next_f64(&mut self) -> f64 {
        self.next_u64() as f64 / (u64::MAX as u128 + 1) as f64
    }

    pub fn entropy() -> u32 {
        Rng::from_entropy().gen::<u32>()
    }

    pub fn entropy_u64() -> u64 {
        Rng::from_entropy().gen::<u64>()
    }

    pub fn entropy_f32() -> f32 {
        (Rng::from_entropy().gen::<u32>() as f64 / (u32::MAX as u64 + 1) as f64) as f32
    }

    pub fn entropy_f64() -> f64 {
        Rng::from_entropy().gen::<u64>() as f64 / (u64::MAX as u128 + 1) as f64
    }

    /// Rolls dice, using the classic 3d6 type of format: n is the number of dice, die_type
    /// is the size of the dice.
    pub fn roll_dice(&mut self, n: u32, dice_type: u32) -> u32 {
        (0..n).map(|_| self.range(1..dice_type + 1)).sum()
    }

    /// Sample a new value, using the given distribution.
    pub fn sample<D>(&mut self) -> D
    where
        rand::distributions::Standard: rand::distributions::Distribution<D>,
    {
        self.rng.sample::<D, _>(Standard)
    }

    pub fn choose<'a, T>(&'a mut self, items: &'a [T]) -> Option<&T> {
        items.choose(&mut self.rng)
    }
}

impl<Rng: RngCore + SeedableRng + Serialize> Iterator for SerializablePrng<Rng> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_u64())
    }
}
