use std::option::Option;

use crate::memory::{Address, AddressRange, MemoryReader};

use super::Deserialize;
use super::Error as DeserializeError;
use super::ptr::{PTR_ALIGNMENT, PTR_NUM_BYTES};

const MAX_STRING_LENGTH: usize = 1024 * 1024;

fn read_c_string<M: MemoryReader>(
    reader: &mut M,
    address: Address,
) -> Result<String, DeserializeError> {
    let mut next_addr = address;

    let mut bytes: Vec<u8> = Vec::new();

    for _ in 0..(MAX_STRING_LENGTH + 1) {
        let byte = reader.read(AddressRange::<1> { start: next_addr })?[0];
        if byte == 0 {
            return Ok(String::from_utf8(bytes)?);
        } else {
            bytes.push(byte);
        }

        match next_addr.checked_add(1) {
            None => break,
            Some(addr) => next_addr = addr,
        };
    }

    Err(DeserializeError::UnterminatedCStringError(address))
}

impl Deserialize for Option<String> {
    const NUM_BYTES: usize = PTR_NUM_BYTES;
    const ALIGNMENT: usize = PTR_ALIGNMENT;

    fn deserialize<M: MemoryReader>(
        reader: &mut M,
        address: Address,
    ) -> Result<Self, DeserializeError> {
        let ptr_range = AddressRange::<PTR_NUM_BYTES> { start: address };
        let addr_raw = usize::from_ne_bytes(reader.read(ptr_range)?);

        Ok(if addr_raw == 0 {
            None
        } else {
            Some(read_c_string(reader, Address::new(addr_raw))?)
        })
    }
}

impl Deserialize for String {
    const NUM_BYTES: usize = PTR_NUM_BYTES;
    const ALIGNMENT: usize = PTR_ALIGNMENT;

    fn deserialize<M: MemoryReader>(
        reader: &mut M,
        address: Address,
    ) -> Result<Self, DeserializeError> {
        match Option::<String>::deserialize(reader, address)? {
            Some(string) => Ok(string),
            None => Err(DeserializeError::NullPtrError(address)),
        }
    }
}
