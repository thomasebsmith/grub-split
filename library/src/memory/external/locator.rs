use std::io;
use std::path::PathBuf;

use proc_maps::{get_process_maps, Pid};

use crate::memory::{Address, MemoryLocator};

struct Library {
    filename: PathBuf,
    starting_address: Address,
}

/// Represents the locations of libraries in an external process's memory at
/// a point in time.
pub struct ExternalMemoryLocator {
    libraries: Vec<Library>,
}

impl ExternalMemoryLocator {
    /// Finds the locations of all libraries in the memory of the process with
    /// ID `pid`.
    ///
    /// This is an expensive operation.
    ///
    /// Locations are captured at the moment this function is called and are
    /// not updated within an [`ExternalMemoryLocator`](ExternalMemoryLocator)
    /// instance.
    ///
    /// Returns an IO error if the libraries could not be located (for example,
    /// because no process with the given PID exists).
    pub fn new(pid: Pid) -> io::Result<Self> {
        let regions = get_process_maps(pid)?;
        let libraries: Vec<Library> = regions
            .iter()
            .filter_map(|region| {
                region.filename().map(|filename| Library {
                    filename: filename.to_path_buf(),
                    starting_address: Address::new(region.start()),
                })
            })
            .collect();
        Ok(Self { libraries })
    }
}

impl MemoryLocator for ExternalMemoryLocator {
    /// Finds the location of the library with the name `library` in the
    /// external process's memory.
    ///
    /// Returns an error if no such library existed at the time this instance
    /// was created.
    fn locate(&mut self, library: &str) -> io::Result<Address> {
        self.libraries
            .iter()
            .filter(|lib| lib.filename.ends_with(library))
            .map(|lib| lib.starting_address)
            .next()
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::Other, "library not found")
            })
    }
}
