use crate::memory::{Address, MemoryReader};

use super::Error as DeserializeError;
use super::{ArrayPtr, Deserialize, LazyDeserialize};

#[derive(Deserialize)]
pub struct PostSizedArray<T: Deserialize, U = usize>
where
    U: Copy + Deserialize + TryInto<usize>,
    DeserializeError: From<<U as TryInto<usize>>::Error>,
{
    array_ptr: Option<ArrayPtr<T>>,
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
        let size = self.size.try_into()?;

        if size == 0 {
            return Ok(vec![]);
        }

        if let Some(ref ptr) = self.array_ptr {
            ptr.deref(reader, size)
        } else {
            // TODO: capture the actual address
            Err(DeserializeError::WithContext(
                Box::new(DeserializeError::NullPtrError(Address::new(0))),
                format!("Null arrayptr in post-sized array with size {size}"),
            ))
        }
    }
}
