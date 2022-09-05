use std::io;

use read_process_memory::{ProcessHandle, copy_address};

use crate::memory::{AddressRange, MemoryReader};

pub struct ExternalMemoryReader {
    handle: ProcessHandle,
}

impl ExternalMemoryReader {
    pub fn from_pid(pid: i32) -> io::Result<Self> {
        Ok(Self { handle: pid.try_into()? })
    }
}

impl MemoryReader for ExternalMemoryReader {
    fn read(&mut self, range: AddressRange) -> io::Result<Vec<u8>> {
        copy_address(range.start.raw(), range.num_bytes, &self.handle)
    }
}
