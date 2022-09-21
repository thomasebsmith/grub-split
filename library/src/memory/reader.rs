use std::io;

use super::address::{AddressRange, VariableLengthAddressRange};

/// Trait for types that can read byte sequences from memory.
pub trait MemoryReader {
    /// Copies the bytes present in `range` into a vector and returns them.
    ///
    /// The exact context for `range` is type-dependent.
    ///
    /// Returns an IO error if the bytes could not be read.
    fn read_vec(
        &mut self,
        range: VariableLengthAddressRange,
    ) -> io::Result<Vec<u8>>;

    /// Copies the bytes present in `range` into a fixed-size array and returns
    /// them.
    ///
    /// The exact context for `range` is type-dependent.
    ///
    /// Returns an IO error if the bytes could not be read.
    ///
    /// The default implementation uses [`read_vec`](MemoryReader::read_vec)
    /// internally; implementations should override this if better performance
    /// is attainable using another method.
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
