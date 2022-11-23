use std::io;

use super::{Address, MemoryReader, VariableLengthAddressRange};

const MACOS_PAGE_SIZE: usize = 4096;

pub struct MemorySearcher<'a> {
    signature: &'a [u8],
    page_size: usize,
}

impl<'a> MemorySearcher<'a> {
    #[must_use]
    pub const fn new(signature: &'a [u8]) -> Self {
        // TODO: check that signature isn't ridiculously long
        Self {
            signature,
            page_size: MACOS_PAGE_SIZE,
        }
    }

    pub fn search<M: MemoryReader>(
        &self,
        reader: &mut M,
        range: VariableLengthAddressRange,
    ) -> io::Result<Option<Address>> {
        let mut addr = range.start;
        let end = range.start + range.num_bytes;
        while addr < end {
            let maybe_data = reader.read_vec(VariableLengthAddressRange {
                start: addr,
                num_bytes: self.signature.len(),
            });

            if let Ok(data) = maybe_data {
                if data == self.signature {
                    return Ok(Some(addr));
                }
            }
            addr = addr + self.page_size;
        }
        Ok(None)
    }
}
