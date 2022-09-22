use core::convert::{From, Infallible};
use std::num::TryFromIntError;
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use std::{error, fmt, io};

use crate::memory::Address;

/// An error that occurs while attempting to deserialize a type from memory.
#[derive(Debug)]
pub enum Error {
    /// Integer overflow occurred on an address while deserializing.
    AddressOverflowError(Address),

    /// A string was not encoded correctly in memory.
    EncodingError(Utf8Error),

    /// An integral type conversion failed.
    IntConversionError(TryFromIntError),

    /// An IO error occurred while attempting to read memory.
    IoError(io::Error),

    /// A pointer was unexpectedly null.
    NullPtrError(Address),

    /// A sequence of characters expected to be null-terminated was not
    /// terminated before reaching an implementation-dependent limit.
    UnterminatedCStringError(Address),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AddressOverflowError(address) => {
                write!(f, "Overflow in address range starting at {}", address)
            }
            Self::EncodingError(encoding_error) => encoding_error.fmt(f),
            Self::IntConversionError(convert_error) => convert_error.fmt(f),
            Self::IoError(io_error) => io_error.fmt(f),
            Self::NullPtrError(address) => {
                write!(f, "Unexpected null pointer at {}", address)
            }
            Self::UnterminatedCStringError(address) => {
                write!(f, "Unterminated C string beginnning at {}", address)
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

impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Self {
        Error::EncodingError(error)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Error::EncodingError(error.utf8_error())
    }
}

impl From<TryFromIntError> for Error {
    fn from(error: TryFromIntError) -> Self {
        Error::IntConversionError(error)
    }
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        panic!("Infallible error encountered");
    }
}
