pub mod external;

mod address;
mod locator;
mod reader;

pub use address::{Address, AddressRange, VariableLengthAddressRange};
pub use locator::MemoryLocator;
pub use reader::MemoryReader;
