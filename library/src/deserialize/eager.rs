use crate::memory::{Address, MemoryReader};

use super::Error as DeserializeError;
use super::{Deserialize, LazyDeserialize};

pub struct Eager<T: LazyDeserialize> {
    pub value: T::Deserialized,
}

impl<T: LazyDeserialize> Deserialize for Eager<T> {
    const NUM_BYTES: usize = T::NUM_BYTES;
    const ALIGNMENT: usize = T::ALIGNMENT;

    fn deserialize<M: MemoryReader>(
        reader: &mut M,
        address: Address,
    ) -> Result<Self, DeserializeError> {
        let lazy_value = T::deserialize(reader, address)?;
        Ok(Self {
            value: lazy_value.deref(reader)?,
        })
    }
}
