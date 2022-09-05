use std::convert::From;
use std::{error, fmt, io};

use crate::memory::Address;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    NullPtrError(Address),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IoError(io_error) => io_error.fmt(f),
            Self::NullPtrError(address) => {
                write!(f, "Unexpected null pointer at {}", address)
            }
        }
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}
