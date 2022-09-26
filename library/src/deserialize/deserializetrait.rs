use std::mem::{align_of, size_of};

use crate::memory::{Address, AddressRange, MemoryReader};

use super::Error as DeserializeError;

pub use grub_split_macros::*;

/// Trait for types that can be deserialized from a fixed-length contiguous byte
/// sequence.
pub trait Deserialize: Sized {
    /// The number of bytes that are required for deserialization.
    const NUM_BYTES: usize;

    /// The required alignment of the bytes. Must be a power of two.
    const ALIGNMENT: usize;

    /// Attempts to deserialize an instance from memory starting at `address`
    /// using `reader`.
    ///
    /// If `address` is not aligned to [`ALIGNMENT`](Deserialize::ALIGNMENT),
    /// behavior is implementation-defined and may be erroneous.
    ///
    /// Returns an [`Error`](super::Error) if deserialization fails.
    fn deserialize<M: MemoryReader>(
        reader: &mut M,
        address: Address,
    ) -> Result<Self, DeserializeError>;
}

macro_rules! deserialize_int_impl {
    ($T:ty) => {
        impl Deserialize for $T {
            const NUM_BYTES: usize = size_of::<$T>();
            const ALIGNMENT: usize = align_of::<$T>();

            fn deserialize<M: MemoryReader>(
                reader: &mut M,
                address: Address,
            ) -> Result<Self, DeserializeError> {
                let range =
                    AddressRange::<{ Self::NUM_BYTES }> { start: address };
                Ok(<$T>::from_ne_bytes(reader.read(range)?))
            }
        }
    };
}

deserialize_int_impl!(u8);
deserialize_int_impl!(i8);
deserialize_int_impl!(u16);
deserialize_int_impl!(i16);
deserialize_int_impl!(u32);
deserialize_int_impl!(i32);
deserialize_int_impl!(u64);
deserialize_int_impl!(i64);
deserialize_int_impl!(usize);
deserialize_int_impl!(isize);

impl Deserialize for bool {
    const NUM_BYTES: usize = 1;
    const ALIGNMENT: usize = 1;

    fn deserialize<M: MemoryReader>(
        reader: &mut M,
        address: Address,
    ) -> Result<Self, DeserializeError> {
        let range = AddressRange::<1> { start: address };
        Ok(reader.read(range)?[0] != 0)
    }
}
