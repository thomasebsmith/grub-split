use std::io;

use super::address::Address;

pub trait MemoryLocator {
    fn locate(&mut self, library: &str) -> io::Result<Address>;
}
