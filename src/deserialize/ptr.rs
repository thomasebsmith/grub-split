use std::marker::PhantomData;
use std::option::Option;

use crate::memory::{Address, AddressRange, MemoryReader};

use super::Deserialize;
use super::Error as DeserializeError;

const PTR_NUM_BYTES: usize = std::mem::size_of::<usize>();

#[derive(Debug)]
pub struct Ptr<T: Deserialize> {
    address: Address,
    deref_type: PhantomData<T>,
}

impl<T: Deserialize> Ptr<T> {
    pub fn deref<M: MemoryReader>(
        self,
        reader: &mut M,
    ) -> Result<T, DeserializeError> {
        T::deserialize(reader, self.address)
    }
}

impl<T: Deserialize> Deserialize for Option<Ptr<T>> {
    fn deserialize<M: MemoryReader>(
        reader: &mut M,
        address: Address,
    ) -> Result<Self, DeserializeError> {
        let range = AddressRange::<PTR_NUM_BYTES> { start: address };
        let addr_raw = usize::from_ne_bytes(reader.read(range)?);
        Ok(if addr_raw == 0 {
            None
        } else {
            Some(Ptr {
                address: Address::new(addr_raw),
                deref_type: PhantomData,
            })
        })
    }
}

impl<T: Deserialize> Deserialize for Ptr<T> {
    fn deserialize<M: MemoryReader>(
        reader: &mut M,
        address: Address,
    ) -> Result<Self, DeserializeError> {
        match Option::<Ptr<T>>::deserialize(reader, address)? {
            Some(addr) => Ok(addr),
            None => Err(DeserializeError::NullPtrError(address)),
        }
    }
}
