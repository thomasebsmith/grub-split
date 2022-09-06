use std::mem::{align_of, size_of};

use crate::memory::{Address, AddressRange, MemoryReader};

use super::Error as DeserializeError;

pub trait Deserialize: Sized {
    const NUM_BYTES: usize;
    const ALIGNMENT: usize;

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
