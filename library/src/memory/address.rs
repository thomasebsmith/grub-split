use std::fmt;
use std::ops::Add;

/// A memory address used to identify a location in another process's memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address(usize);

impl Address {
    /// Constructs a new `Address` referring to the byte offset `raw` in another
    /// process's virtual memory.
    #[must_use]
    pub const fn new(raw: usize) -> Self {
        Self(raw)
    }

    /// Gets the byte offset in virtual memory that this address represents.
    #[must_use]
    pub const fn raw(self) -> usize {
        self.0
    }

    /// Constructs a new `Address` that is aligned to `alignment` by increasing
    /// this address's internal virtual memory offset.
    ///
    /// `alignment` must be a power of 2.
    #[must_use]
    pub const fn align_forward(self, alignment: usize) -> Self {
        let alignment_mask = alignment - 1;
        let addr_in_correct_block = self.0 + alignment_mask;
        let addr_aligned = addr_in_correct_block & !alignment_mask;
        Self(addr_aligned)
    }

    // const_ops would be nice...
    /// Constructs a new `Address` that is `offset` bytes forward in memory from
    /// this `Address`.
    #[must_use]
    pub const fn add_const(self, offset: usize) -> Self {
        Self(self.0 + offset)
    }

    /// Attempts to create a new `Address` that is `offset` bytes forward in
    /// memory from this `Address`, checking for overflow.
    ///
    /// Returns `None` if overflow would have occured or `Some(address)` if the
    /// addition was successful.
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

/// A constant-length, contiguous range of bytes in another process's memory.
///
/// `NUM_BYTES` is the number of bytes included in the `AddressRange`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AddressRange<const NUM_BYTES: usize> {
    /// The address where this range begins.
    pub start: Address,
}

impl<const NUM_BYTES: usize> AddressRange<NUM_BYTES> {
    /// Checks whether `other` is contained entirely within this `AddressRange`.
    ///
    /// This will return true if and only if `other` starts no later than this
    /// range and ends no earlier than this range.
    #[must_use]
    pub fn contains<const OTHER_NUM_BYTES: usize>(
        self,
        other: AddressRange<OTHER_NUM_BYTES>,
    ) -> bool {
        other.start >= self.start
            && other.start + OTHER_NUM_BYTES <= self.start + NUM_BYTES
    }
}

/// A variable-length, contiguous range of bytes in another process's memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VariableLengthAddressRange {
    /// The address where this range begins.
    pub start: Address,

    /// The number of bytes contained within this range.
    pub num_bytes: usize,
}

impl VariableLengthAddressRange {
    /// Checks whether `other` is contained entirely within this `AddressRange`.
    ///
    /// This will return true if and only if `other` starts no later than this
    /// range and ends no earlier than this range.
    #[must_use]
    pub fn contains(self, other: Self) -> bool {
        other.start >= self.start
            && other.start + other.num_bytes <= self.start + self.num_bytes
    }
}
