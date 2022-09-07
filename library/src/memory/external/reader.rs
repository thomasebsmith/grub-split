use std::io;

use read_process_memory::{copy_address, CopyAddress, ProcessHandle};

use crate::memory::{AddressRange, MemoryReader, VariableLengthAddressRange};

pub struct ExternalMemoryReader {
    handle: ProcessHandle,
}

impl ExternalMemoryReader {
    pub fn from_pid(pid: i32) -> io::Result<Self> {
        Ok(Self {
            handle: pid.try_into()?,
        })
    }
}

impl MemoryReader for ExternalMemoryReader {
    fn read_vec(
        &mut self,
        range: VariableLengthAddressRange,
    ) -> io::Result<Vec<u8>> {
        copy_address(range.start.raw(), range.num_bytes, &self.handle)
    }

    fn read<const NUM_BYTES: usize>(
        &mut self,
        range: AddressRange<NUM_BYTES>,
    ) -> io::Result<[u8; NUM_BYTES]> {
        let mut result: [u8; NUM_BYTES] = [0; NUM_BYTES];
        self.handle.copy_address(range.start.raw(), &mut result)?;
        Ok(result)
    }
}
