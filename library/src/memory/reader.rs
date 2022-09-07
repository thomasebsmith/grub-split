use std::io;

use super::address::{Address, AddressRange, VariableLengthAddressRange};

pub trait MemoryLocator {
    fn locate(&mut self, library: &str) -> io::Result<Address>;
}

pub trait MemoryReader {
    fn read_vec(
        &mut self,
        range: VariableLengthAddressRange,
    ) -> io::Result<Vec<u8>>;

    fn read<const NUM_BYTES: usize>(
        &mut self,
        range: AddressRange<NUM_BYTES>,
    ) -> io::Result<[u8; NUM_BYTES]> {
        Ok(self
            .read_vec(VariableLengthAddressRange {
                start: range.start,
                num_bytes: NUM_BYTES,
            })?
            .try_into()
            .unwrap())
    }
}
