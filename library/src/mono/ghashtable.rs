// See mono/eglib/ghashtable.c in Mono

// _funcs are actually function pointers, but they are deserialized as usizes
// since we don't need to use them.

use std::collections::LinkedList;

use crate::deserialize::{Deserialize, Eager, PostSizedArray, Ptr};

use super::Hash as MonoHash;

#[derive(Deserialize)]
struct KeyValuePair<K: Deserialize + MonoHash + Eq, V: Deserialize> {
    key: K,
    value: V,
}

type Slot<K, V> = LinkedList<KeyValuePair<K, V>>;

type SlotPtr<K, V> = Option<Ptr<Slot<K, V>>>;

type SlotArray<K, V> = Eager<PostSizedArray<Eager<SlotPtr<K, V>>, i32>>;

#[derive(Deserialize)]
pub struct GHashTable<K: Deserialize + MonoHash + Eq, V: Deserialize> {
    _hash_func: usize,
    _key_equal_func: usize,
    table: SlotArray<K, V>,
    _in_use: i32,
    _threshold: i32,
    _last_rehash: i32,
    _value_destroy_func: usize,
    _key_destroy_func: usize,
}

impl<K: Deserialize + MonoHash + Eq, V: Deserialize> GHashTable<K, V> {
    pub fn get(&self, key: K) -> Option<&V> {
        let bucket = (key.hash() as usize) % self.table.value.len();
        let maybe_slot = &self.table.value[bucket].value;
        match maybe_slot {
            None => None,
            Some(slot) => {
                for pair in slot.iter() {
                    if pair.key == key {
                        return Some(&pair.value);
                    }
                }

                None
            }
        }
    }
}
