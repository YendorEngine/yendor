pub trait RandomValue: Copy {
    fn to_u64(self) -> u64;
    fn from_u64(value: u64) -> Self;
}

impl RandomValue for u64 {
    #[inline]
    fn to_u64(self) -> u64 { self }

    #[inline]
    fn from_u64(value: u64) -> Self { value }
}

impl RandomValue for u32 {
    #[inline]
    fn to_u64(self) -> u64 { self as u64 }

    #[inline]
    fn from_u64(value: u64) -> Self { value as u32 }
}

impl RandomValue for usize {
    #[inline]
    fn to_u64(self) -> u64 { self as u64 }

    #[inline]
    fn from_u64(value: u64) -> Self { value as usize }
}
