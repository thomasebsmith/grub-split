use std::error::Error;
use std::io;

use log::{debug, trace};

use grub_split_library::deserialize::LazyDeserialize;
use grub_split_library::memory::external::{
    ExternalMemoryLocator, ExternalMemoryReader,
};
use grub_split_library::mono::{LoadedImages, MONO_TOKEN_TYPE_DEF};

fn to_usize<T: TryInto<usize>>(
    value: T,
) -> Result<usize, <T as TryInto<usize>>::Error> {
    value.try_into()
}

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
    let type_token =
        to_usize(*ns_cache.value.get("GameManager").ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Other,
                "GameManager type token not found",
            )
        })?)?;
    trace!("type token = {:x}", type_token);
    let type_def_token = type_token + MONO_TOKEN_TYPE_DEF;
    trace!("type def token = {:x}", type_def_token);

    let class_cache_size = to_usize(image.class_cache.size)?;
    let mut class = image
        .class_cache
        .table
        .nth_element(&mut reader, type_def_token % class_cache_size)?
        .value;
    while to_usize(class.type_token)? != type_def_token {
        trace!(
            "Found class {} with type def token {:x}",
            &class.name,
            class.type_token
        );
        let Some(ptr) = class.next_class_cache else {
            return Err(Box::new(io::Error::new(io::ErrorKind::Other, "Class cache entry not found")));
        };
        class = ptr.deref(&mut reader)?;
    }
    debug!("Found class with name {}", &class.name);

    Ok(())
}
