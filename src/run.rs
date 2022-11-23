use std::error::Error;
use std::io;

use log::{debug, trace};

use grub_split_library::memory::external::{
    ExternalMemoryLocator, ExternalMemoryReader,
};
use grub_split_library::mono::LoadedImages;

pub fn run(pid: i32) -> Result<(), Box<dyn Error>> {
    trace!("Attaching to process");
    let mut locator = ExternalMemoryLocator::new(pid)?;
    let mut reader = ExternalMemoryReader::from_pid(pid)?;

    trace!("Finding loaded images");
    let loaded_images = LoadedImages::new(&mut locator, &mut reader)?;
    trace!("Found loaded images");
    let image =
        loaded_images.get_image("Assembly-CSharp").ok_or_else(|| {
            io::Error::new(io::ErrorKind::Other, "Image not found")
        })?;
    trace!("Found image");
    let ns_cache = image
        .name_cache
        .value
        .as_ref()
        .ok_or_else(|| {
            io::Error::new(io::ErrorKind::Other, "Null namespace cache")
        })?
        .get("")
        .ok_or_else(|| {
            io::Error::new(io::ErrorKind::Other, "Empty namespace not found")
        })?;
    trace!("Found ns cache");
    let type_token = ns_cache.value.get("GameManager").ok_or_else(|| {
        io::Error::new(io::ErrorKind::Other, "GameManager type token not found")
    })?;

    debug!("name = {}", image.name);
    debug!("type token = {}", type_token);

    Ok(())
}
