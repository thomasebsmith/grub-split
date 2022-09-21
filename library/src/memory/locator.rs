use std::io;

use super::address::Address;

/// Trait for types that can locate a library's starting address in memory.
pub trait MemoryLocator {
    /// Finds the address at which `library` begins in virtual memory in a
    /// type-dependent context.
    ///
    /// Returns the library's starting address in virtual memory if it was
    /// found.
    ///
    /// Returns an IO error if the library could not be found.
    fn locate(&mut self, library: &str) -> io::Result<Address>;
}
