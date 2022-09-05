use std::fmt;
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address(usize);

impl Address {
    pub fn new(raw: usize) -> Self {
        Self(raw)
    }

    pub fn raw(self) -> usize {
        self.0
    }
}

impl Add<usize> for Address {
    type Output = Self;

    fn add(self, offset: usize) -> Self {
        Self(self.0 + offset)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AddressRange<const NUM_BYTES: usize> {
    pub start: Address,
}

impl<const NUM_BYTES: usize> AddressRange<NUM_BYTES> {
    pub fn contains<const OTHER_NUM_BYTES: usize>(
        self,
        other: AddressRange<OTHER_NUM_BYTES>,
    ) -> bool {
        other.start >= self.start
            && other.start + OTHER_NUM_BYTES <= self.start + NUM_BYTES
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VariableLengthAddressRange {
    pub start: Address,
    pub num_bytes: usize,
}

impl VariableLengthAddressRange {
    pub fn contains(self, other: Self) -> bool {
        other.start >= self.start
            && other.start + other.num_bytes <= self.start + self.num_bytes
    }
}
