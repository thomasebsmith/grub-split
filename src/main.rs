use std::error::Error;
use std::io;

use log::{debug, trace};

use grub_split_library::memory::external::{
    ExternalMemoryLocator, ExternalMemoryReader,
};
use grub_split_library::mono::LoadedImages;

fn invalid_input(desc: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, desc)
}

fn usage(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        eprintln!("Usage: <executable> <pid>");
    } else {
        eprintln!("Usage: {} <pid>", args[0]);
    }
    Err(invalid_input("Invalid arguments"))
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        usage(args.as_slice())?;
    }

    let pid: i32 = args[1].parse()?;
    let mut locator = ExternalMemoryLocator::new(pid)?;
    let mut reader = ExternalMemoryReader::from_pid(pid)?;

    trace!("Starting...");
    let loaded_images = LoadedImages::new(&mut locator, &mut reader)?;
    trace!("Found loaded images");
    let image =
        loaded_images.get_image("Assembly-CSharp").ok_or_else(|| {
            io::Error::new(io::ErrorKind::Other, "Image not found")
        })?;
    trace!("Found image");
    let ns_cache = image.name_cache.value.get("").ok_or_else(|| {
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
