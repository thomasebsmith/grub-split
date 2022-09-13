use std::io;
use std::path::PathBuf;

use proc_maps::{get_process_maps, Pid};

use crate::memory::{Address, MemoryLocator};

struct Library {
    filename: PathBuf,
    starting_address: Address,
}

pub struct ExternalMemoryLocator {
    libraries: Vec<Library>,
}

impl ExternalMemoryLocator {
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
