use crate::deserialize::{ArrayPtr, Deserialize};
use crate::memory::Address;

#[derive(Debug, Deserialize)]
pub struct MonoInternalHashTable<T: Deserialize> {
    pub hash_func: Option<Address>,
    pub key_extract: Option<Address>,
    pub next_value: Option<Address>,
    pub size: i32,
    pub num_entries: i32,
    pub table: ArrayPtr<T>,
}
