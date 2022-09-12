use std::collections::LinkedList;

use crate::memory::{Address, MemoryReader};

use super::ptr::{PTR_ALIGNMENT, PTR_NUM_BYTES};
use super::Deserialize;
use super::Error as DeserializeError;

#[derive(Deserialize)]
struct Node<T> {
    value: T,
    next: usize,
}

impl<T: Deserialize> Deserialize for LinkedList<T> {
    const NUM_BYTES: usize = PTR_NUM_BYTES;
    const ALIGNMENT: usize = PTR_ALIGNMENT;

    fn deserialize<M: MemoryReader>(
        reader: &mut M,
        address: Address,
    ) -> Result<Self, DeserializeError> {
        // TODO: check for loops

        let mut result = LinkedList::<T>::new();

        let mut next_node_addr = address;
        while next_node_addr.raw() != 0 {
            let node = Node::<T>::deserialize(reader, next_node_addr)?;
            result.push_back(node.value);
            next_node_addr = Address::new(node.next);
        }

        Ok(result)
    }
}
