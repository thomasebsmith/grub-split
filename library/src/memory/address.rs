use std::fmt;
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address(usize);

impl Address {
    #[must_use]
    pub const fn new(raw: usize) -> Self {
        Self(raw)
    }

    #[must_use]
    pub const fn raw(self) -> usize {
        self.0
    }

    #[must_use]
    pub const fn align_forward(self, alignment: usize) -> Self {
        let alignment_mask = alignment - 1;
        let addr_in_correct_block = self.0 + alignment_mask;
        let addr_aligned = addr_in_correct_block & !alignment_mask;
        Self(addr_aligned)
    }

    // const_ops would be nice...
    #[must_use]
    pub const fn add_const(self, offset: usize) -> Self {
        Self(self.0 + offset)
    }

    #[must_use]
    pub const fn checked_add(self, offset: usize) -> Option<Self> {
        match self.0.checked_add(offset) {
            None => None,
            Some(new_raw_addr) => Some(Self(new_raw_addr)),
        }
    }
}

impl Add<usize> for Address {
    type Output = Self;

    fn add(self, offset: usize) -> Self {
        self.add_const(offset)
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
    #[must_use]
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
    #[must_use]
    pub fn contains(self, other: Self) -> bool {
        other.start >= self.start
            && other.start + other.num_bytes <= self.start + self.num_bytes
    }
}
