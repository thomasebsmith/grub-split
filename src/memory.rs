pub mod external;

mod address;
mod reader;

pub use address::{Address, AddressRange, VariableLengthAddressRange};
pub use reader::{MemoryLocator, MemoryReader};
