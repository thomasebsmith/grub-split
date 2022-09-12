use std::marker::PhantomData;

use crate::memory::{Address, AddressRange, MemoryReader};

use super::Deserialize;
use super::Error as DeserializeError;
use super::LazyDeserialize;

pub const PTR_NUM_BYTES: usize = std::mem::size_of::<usize>();
pub const PTR_ALIGNMENT: usize = std::mem::align_of::<usize>();

pub fn read_ptr<M: MemoryReader>(
    reader: &mut M,
    address: Address,
) -> Result<Option<Address>, DeserializeError> {
    let range = AddressRange::<PTR_NUM_BYTES> { start: address };
    let addr_raw = usize::from_ne_bytes(reader.read(range)?);
    Ok(if addr_raw == 0 {
        None
    } else {
        Some(Address::new(addr_raw))
    })
}

#[derive(Debug)]
pub struct Ptr<T: Deserialize> {
    address: Address,
    deref_type: PhantomData<T>,
}

impl<T: Deserialize> LazyDeserialize for Ptr<T> {
    type Deserialized = T;

    fn deref<M: MemoryReader>(
        &self,
        reader: &mut M,
    ) -> Result<Self::Deserialized, DeserializeError> {
        T::deserialize(reader, self.address)
    }
}

impl<T: Deserialize> Deserialize for Option<Ptr<T>> {
    const NUM_BYTES: usize = PTR_NUM_BYTES;
    const ALIGNMENT: usize = PTR_ALIGNMENT;

    fn deserialize<M: MemoryReader>(
        reader: &mut M,
        address: Address,
    ) -> Result<Self, DeserializeError> {
        Ok(read_ptr(reader, address)?.map(|pointed_addr| Ptr {
            address: pointed_addr,
            deref_type: PhantomData,
        }))
    }
}

impl<T: Deserialize> Deserialize for Ptr<T> {
    const NUM_BYTES: usize = PTR_NUM_BYTES;
    const ALIGNMENT: usize = PTR_ALIGNMENT;

    fn deserialize<M: MemoryReader>(
        reader: &mut M,
        address: Address,
    ) -> Result<Self, DeserializeError> {
        match Option::<Ptr<T>>::deserialize(reader, address)? {
            Some(ptr) => Ok(ptr),
            None => Err(DeserializeError::NullPtrError(address)),
        }
    }
}
