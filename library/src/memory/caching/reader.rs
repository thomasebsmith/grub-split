use std::cmp::{max, min};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::io;

use crate::memory::{
    Address, AddressRange, MemoryReader, VariableLengthAddressRange,
};

pub struct CachingMemoryReader<M: MemoryReader, const PAGE_SIZE: usize> {
    reader: M,
    cache: HashMap<usize, [u8; PAGE_SIZE]>,
}

impl<M: MemoryReader, const PAGE_SIZE: usize>
    CachingMemoryReader<M, PAGE_SIZE>
{
    pub fn new(reader: M) -> Self {
        Self {
            reader,
            cache: HashMap::new(),
        }
    }

    fn add_page_to_cache(
        &mut self,
        page_num: usize,
    ) -> io::Result<&[u8; PAGE_SIZE]> {
        Ok(match self.cache.entry(page_num) {
            Occupied(occupied) => occupied.into_mut(),
            Vacant(vacant) => {
                vacant.insert(self.reader.read(AddressRange::<PAGE_SIZE> {
                    start: Address::new(page_num * PAGE_SIZE),
                })?)
            }
        })
    }
}

impl<M: MemoryReader, const PAGE_SIZE: usize> MemoryReader
    for CachingMemoryReader<M, PAGE_SIZE>
{
    fn read_vec(
        &mut self,
        range: VariableLengthAddressRange,
    ) -> io::Result<Vec<u8>> {
        let mut result = Vec::<u8>::with_capacity(range.num_bytes);

        if range.num_bytes == 0 {
            return Ok(result);
        }

        let range_start = range.start.raw();
        let range_end = range_start + range.num_bytes;

        let first_page_num = range_start / PAGE_SIZE;
        let last_page_num =
            first_page_num + (range.num_bytes - 1) / PAGE_SIZE + 1;

        for page_num in first_page_num..last_page_num {
            let page_data = self.add_page_to_cache(page_num)?;
            let start_offset =
                max(page_num * PAGE_SIZE, range_start) % PAGE_SIZE;
            let mut end_offset =
                min((page_num + 1) * PAGE_SIZE, range_end) % PAGE_SIZE;
            if end_offset == 0 {
                end_offset = PAGE_SIZE;
            }
            result.extend_from_slice(&page_data[start_offset..end_offset]);
        }

        Ok(result)
    }
}
