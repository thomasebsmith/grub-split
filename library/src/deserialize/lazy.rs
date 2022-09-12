use crate::memory::MemoryReader;

use super::Deserialize;
use super::Error as DeserializeError;

pub trait LazyDeserialize: Deserialize {
    type Deserialized;

    fn deref<M: MemoryReader>(
        &self,
        reader: &mut M,
    ) -> Result<Self::Deserialized, DeserializeError>;
}

impl<T: LazyDeserialize> LazyDeserialize for Option<T>
where
    Option<T>: Deserialize,
{
    type Deserialized = Option<T::Deserialized>;

    fn deref<M: MemoryReader>(
        &self,
        reader: &mut M,
    ) -> Result<Self::Deserialized, DeserializeError> {
        Ok(match self {
            None => None,
            Some(value) => Some(value.deref(reader)?),
        })
    }
}
