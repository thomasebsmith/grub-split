use crate::memory::{Address, MemoryReader};

use super::Deserialize;
use super::Error as DeserializeError;

impl<T: Deserialize, const N: usize> Deserialize for [T; N] {
    const NUM_BYTES: usize = T::NUM_BYTES * N;
    const ALIGNMENT: usize = T::ALIGNMENT;

    fn deserialize<M: MemoryReader>(
        reader: &mut M,
        address: Address,
    ) -> Result<Self, DeserializeError> {
        let padded_element_size: usize =
            Address::new(T::NUM_BYTES).align_forward(T::ALIGNMENT).raw();

        std::array::try_from_fn(|index| {
            T::deserialize(reader, address + index * padded_element_size)
        })
    }
}
