use std::ops::Add;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(PartialEq, Eq)]
pub struct AddressRange {
    pub start: Address,
    pub num_bytes: usize,
}

impl AddressRange {
    pub fn contains(self, other: Self) -> bool {
        other.start >= self.start &&
            other.start + other.num_bytes <= self.start + self.num_bytes
    }
}
