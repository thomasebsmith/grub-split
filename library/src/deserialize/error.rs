use core::convert::{From, Infallible};
use std::num::TryFromIntError;
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use std::{error, fmt, io};

use crate::memory::Address;

#[derive(Debug)]
pub enum Error {
    AddressOverflowError(Address),
    EncodingError(Utf8Error),
    IntConversionError(TryFromIntError),
    IoError(io::Error),
    NullPtrError(Address),
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
