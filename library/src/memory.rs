pub mod external;

mod address;
mod locator;
mod reader;
mod searcher;

pub use address::{Address, AddressRange, VariableLengthAddressRange};
pub use locator::MemoryLocator;
pub use reader::MemoryReader;
pub use searcher::MemorySearcher;
