use crate::memory::MemoryReader;

use super::Error as DeserializeError;

pub trait LazyDeserialize<T> {
    fn deref<M: MemoryReader>(
        &self,
        reader: &mut M,
    ) -> Result<T, DeserializeError>;
}
