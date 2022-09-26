use std::marker::PhantomData;

use crate::memory::{Address, MemoryReader};

use super::ptr::{PTR_ALIGNMENT, PTR_NUM_BYTES};
use super::Deserialize;
use super::Error as DeserializeError;

#[derive(Debug)]
pub struct ArrayPtr<T: Deserialize> {
    address: Address,
    deref_type: PhantomData<T>,
}

impl<T: Deserialize> ArrayPtr<T> {
    pub fn nth_element<M: MemoryReader>(
        &self,
        reader: &mut M,
        index: usize,
    ) -> Result<T, DeserializeError> {
        let padded_element_size: usize =
            Address::new(T::NUM_BYTES).align_forward(T::ALIGNMENT).raw();

        T::deserialize(reader, self.address + index * padded_element_size)
    }

    pub fn deref<M: MemoryReader>(
        &self,
        reader: &mut M,
        size: usize,
    ) -> Result<Vec<T>, DeserializeError> {
        (0..size)
            .map(|index| self.nth_element(reader, index))
            .collect()
    }
}

impl<T: Deserialize> Deserialize for Option<ArrayPtr<T>> {
    const NUM_BYTES: usize = PTR_NUM_BYTES;
    const ALIGNMENT: usize = PTR_ALIGNMENT;

    fn deserialize<M: MemoryReader>(
        reader: &mut M,
        address: Address,
    ) -> Result<Self, DeserializeError> {
        Ok(Option::<Address>::deserialize(reader, address)?.map(
            |pointed_addr| ArrayPtr {
                address: pointed_addr,
                deref_type: PhantomData,
            },
        ))
    }
}

impl<T: Deserialize> Deserialize for ArrayPtr<T> {
    const NUM_BYTES: usize = PTR_NUM_BYTES;
    const ALIGNMENT: usize = PTR_ALIGNMENT;

    fn deserialize<M: MemoryReader>(
        reader: &mut M,
        address: Address,
    ) -> Result<Self, DeserializeError> {
        match Option::<ArrayPtr<T>>::deserialize(reader, address)? {
            Some(ptr) => Ok(ptr),
            None => Err(DeserializeError::NullPtrError(address)),
        }
    }
}
