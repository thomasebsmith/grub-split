pub mod external;

mod address;
mod reader;

pub use address::{Address, AddressRange};
pub use reader::{MemoryLocator, MemoryReader};
