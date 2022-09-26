use crate::deserialize::Deserialize;
use crate::memory::Address;

#[derive(Deserialize)]
pub struct MonoInternalHashTable {
    pub hash_func: Option<Address>,
    pub key_extract: Option<Address>,
    pub next_value: Option<Address>,
    pub size: i32,
    pub num_entries: i32,
    pub table: Option<Address>,
}
