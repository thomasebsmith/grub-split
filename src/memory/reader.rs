use std::io;

use super::address::{Address, AddressRange};

pub trait MemoryLocator {
    fn locate(&mut self, library: &str) -> io::Result<Address>;
}

pub trait MemoryReader {
    fn read(&mut self, range: AddressRange) -> io::Result<Vec<u8>>;
}
