use crate::memory::{Address, MemoryReader};

use super::Error as DeserializeError;
use super::{ArrayPtr, Deserialize};

pub struct ZeroLengthArray<T: Deserialize> {
    array_ptr: ArrayPtr<T>,
}

impl<T: Deserialize> ZeroLengthArray<T> {
    pub fn nth_element<M: MemoryReader>(
        &self,
        reader: &mut M,
        index: usize,
    ) -> Result<T, DeserializeError> {
        self.array_ptr.nth_element(reader, index)
    }

    pub fn deref<M: MemoryReader>(
        &self,
        reader: &mut M,
        size: usize,
    ) -> Result<Vec<T>, DeserializeError> {
        self.array_ptr.deref(reader, size)
    }
}

impl<T: Deserialize> Deserialize for ZeroLengthArray<T> {
    const NUM_BYTES: usize = 0;
    const ALIGNMENT: usize = T::ALIGNMENT;

    fn deserialize<M: MemoryReader>(
        _reader: &mut M,
        address: Address,
    ) -> Result<Self, DeserializeError> {
        let array_ptr = ArrayPtr::<T>::new(address.align_forward(T::ALIGNMENT));
        Ok(Self { array_ptr })
    }
}
