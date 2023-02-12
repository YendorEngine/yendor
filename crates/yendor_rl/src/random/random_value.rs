/// Trait for types that can be used as random values.
pub trait RandomValue: Copy {
    /// Convert the value to a u64.
    fn to_u64(self) -> u64;

    /// Convert a u64 to the value.
    fn from_u64(value: u64) -> Self;
}

impl RandomValue for u64 {
    #[inline]
    fn to_u64(self) -> u64 {
        self
    }

    #[inline]
    fn from_u64(value: u64) -> Self {
        value
    }
}

impl RandomValue for u32 {
    #[inline]
    fn to_u64(self) -> u64 {
        self as u64
    }

    #[inline]
    fn from_u64(value: u64) -> Self {
        value as Self
    }
}

impl RandomValue for usize {
    #[inline]
    fn to_u64(self) -> u64 {
        self as u64
    }

    #[inline]
    fn from_u64(value: u64) -> Self {
        value as Self
    }
}
