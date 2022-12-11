use std::ops::{Bound, Index, IndexMut, RangeBounds};

use crate::prelude::*;

pub trait Random {
    fn next_u32(&mut self) -> u32;
    fn next_u64(&mut self) -> u64;
    fn fill_bytes(&mut self, dest: &mut [u8]);
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error>;

    fn coin(&mut self) -> bool { self.max_inclusive(1u64) == 0 }

    fn max<Value: RandomValue>(&mut self, value: Value) -> Value {
        let value = value.to_u64();
        if value == 0 { Value::from_u64(0) } else { self.max_inclusive(Value::from_u64(value - 1)) }
    }

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
                x = self.next_u64() as u128;
                if x < limit {
                    break;
                }
            }
            Value::from_u64((x / buckets) as u64)
        }
    }

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

    fn float(&mut self) -> f64 { self.next_u64() as f64 / (u64::MAX as u128 + 1) as f64 }

    fn index<'a, T: Index<usize>>(&mut self, items: &'a [T]) -> Option<&'a T> {
        let len = items.len();
        if len == 0 { None } else { items.get(self.range(0..len)) }
    }

    fn index_mut<'a, T: IndexMut<usize>>(&mut self, items: &'a mut [T]) -> Option<&'a mut T> {
        let len = items.len();
        if len == 0 { None } else { items.get_mut(self.range(0..len)) }
    }

    fn one_in<Value: RandomValue>(&mut self, value: Value) -> bool { self.max(value).to_u64() == 0 }

    // TODO: roll possible roll settings instead of generics?
}

impl<T: RngCore> Random for T {
    fn next_u32(&mut self) -> u32 { self.next_u32() }

    fn next_u64(&mut self) -> u64 { self.next_u64() }

    fn fill_bytes(&mut self, dest: &mut [u8]) { self.fill_bytes(dest) }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> { self.try_fill_bytes(dest) }
}
