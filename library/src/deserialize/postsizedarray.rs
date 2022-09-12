use crate::memory::MemoryReader;

use super::Error as DeserializeError;
use super::{ArrayPtr, Deserialize, LazyDeserialize};

#[derive(Deserialize)]
pub struct PostSizedArray<T: Deserialize, U = usize>
where
    U: Copy + Deserialize + TryInto<usize>,
    DeserializeError: From<<U as TryInto<usize>>::Error>,
{
    array_ptr: ArrayPtr<T>,
    size: U,
}

impl<T: Deserialize, U: Copy + Deserialize + TryInto<usize>> LazyDeserialize
    for PostSizedArray<T, U>
where
    DeserializeError: From<<U as TryInto<usize>>::Error>,
{
    type Deserialized = Vec<T>;

    fn deref<M: MemoryReader>(
        &self,
        reader: &mut M,
    ) -> Result<Self::Deserialized, DeserializeError> {
        self.array_ptr.deref(reader, self.size.try_into()?)
    }
}
