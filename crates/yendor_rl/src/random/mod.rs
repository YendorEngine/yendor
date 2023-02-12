//! Provides an interface for random number generation.
use std::ops::{Bound, Index, IndexMut, RangeBounds};

use crate::prelude::*;

mod random_value;
pub use random_value::*;

/// A trait to provide random values.
pub trait Rand {
    /// Returns the next random u32.
    fn internal_next_u32(&mut self) -> u32;
    /// Returns the next random u64.
    fn internal_next_u64(&mut self) -> u64;
    /// Fills the provided buffer with random bytes.
    fn internal_fill_bytes(&mut self, dest: &mut [u8]);
    /// Tries to fill the provided buffer with random bytes.
    fn internal_try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error>;

    /// Flip a coin.
    fn coin(&mut self) -> bool {
        self.max_inclusive(1u64) == 0
    }

    /// Returns a random value in the range [0, value).
    fn max<Value: RandomValue>(&mut self, value: Value) -> Value {
        let value = value.to_u64();
        if value == 0 {
            Value::from_u64(0)
        } else {
            self.max_inclusive(Value::from_u64(value - 1))
        }
    }

    /// Returns a random value in the range [0, value].
    fn max_inclusive<Value: RandomValue>(&mut self, value: Value) -> Value {
        let value = value.to_u64();
        if value == 0 {
            Value::from_u64(0)
        } else {
            let top = value as u128 + 1;
            let buckets = (u64::MAX as u128 / top).max(1);
            let limit = (buckets * top).max(1);
            let mut x;
            loop {
                x = self.internal_next_u64() as u128;
                if x < limit {
                    break;
                }
            }
            Value::from_u64((x / buckets) as u64)
        }
    }

    /// Returns a random value in the range [min, max].
    fn range<Value: RandomValue, R: RangeBounds<Value>>(&mut self, range: R) -> Value {
        let start = match range.start_bound() {
            Bound::Included(&start) => start.to_u64(),
            Bound::Excluded(&start) => start.to_u64() + 1,
            Bound::Unbounded => u64::MIN,
        };
        let end = match range.end_bound() {
            Bound::Included(&end) => end.to_u64() + 1,
            Bound::Excluded(&end) => end.to_u64(),
            Bound::Unbounded => u64::MAX,
        };

        let min = start.min(end);
        let difference = start.max(end) - min;

        Value::from_u64(self.max_inclusive(difference) + min)
    }

    /// Returns a random f64
    fn float(&mut self) -> f64 {
        self.internal_next_u64() as f64 / (u64::MAX as u128 + 1) as f64
    }

    /// Returns a random item from the provided slice.
    fn index<'a, T: Index<usize>>(&mut self, items: &'a [T]) -> Option<&'a T> {
        let len = items.len();
        if len == 0 {
            None
        } else {
            items.get(self.range(0..len))
        }
    }

    /// Returns a random mutable item from the provided slice.
    fn index_mut<'a, T: IndexMut<usize>>(&mut self, items: &'a mut [T]) -> Option<&'a mut T> {
        let len = items.len();
        if len == 0 {
            None
        } else {
            items.get_mut(self.range(0..len))
        }
    }

    /// Returns true if the random value is less than the provided value.
    fn one_in<Value: RandomValue>(&mut self, value: Value) -> bool {
        self.max(value).to_u64() == 0
    }

    // TODO: roll possible roll settings instead of generics?
}

impl<T: RngCore> Rand for T {
    fn internal_next_u32(&mut self) -> u32 {
        self.next_u32()
    }

    fn internal_next_u64(&mut self) -> u64 {
        self.next_u64()
    }

    fn internal_fill_bytes(&mut self, dest: &mut [u8]) {
        self.fill_bytes(dest)
    }

    fn internal_try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.try_fill_bytes(dest)
    }
}
