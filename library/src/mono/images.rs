use crate::deserialize::Error as DeserializeError;
use crate::deserialize::{Deserialize, Eager, Ptr};
use crate::memory::{MemoryLocator, MemoryReader};

use super::GHashTable;

const MONO_LIBRARY_NAME: &str = "libmonobdwgc-2.0.dylib";
const LOADED_IMAGES_OFFSET: usize = 0x0016_d638 + 0x0018_e978 + 0x10;

#[derive(Deserialize)]
pub struct Image {
    _ref_count: i32,
    _raw_data_handle: usize,
    _raw_data: usize,
    _raw_data_len: u32,
    _bitfields: [u8; 2],
    pub name: String,
    pub assembly_name: Option<String>,
    pub module_name: Option<String>,
    // ...
}

type ImageHashTable = Eager<Ptr<GHashTable<String, Eager<Ptr<Image>>>>>;

pub struct LoadedImages {
    loaded_images_by_name: ImageHashTable,
}

impl LoadedImages {
    pub fn new<L: MemoryLocator, M: MemoryReader>(
        locator: &mut L,
        reader: &mut M,
    ) -> Result<Self, DeserializeError> {
        Ok(Self {
            loaded_images_by_name: ImageHashTable::deserialize(
                reader,
                locator.locate(MONO_LIBRARY_NAME)? + LOADED_IMAGES_OFFSET,
            )?,
        })
    }

    #[must_use]
    pub fn get_image(&self, name: &str) -> Option<&Image> {
        // TODO: this shouldn't be necessary
        let name_string = String::from(name);

        let eager_ptr = self.loaded_images_by_name.value.get(&name_string)?;
        Some(&eager_ptr.value)
    }
}
