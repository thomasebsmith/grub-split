use crate::deserialize::Error as DeserializeError;
use crate::deserialize::{Deserialize, Eager, Ptr};
use crate::memory::{MemoryLocator, MemoryReader};

use super::GHashTable;

const MONO_LIBRARY_NAME: &str = "libmonobdwgc-2.0.dylib";
const LOADED_IMAGES_OFFSET: usize = 0x0016_d638 + 0x0018_e978 + 0x10;

fn has_flag(byte: u8, flag: u8) -> bool {
    byte & flag == flag
}

#[derive(Deserialize)]
pub struct MonoStreamHeader {
    pub data: Option<Address>,
    pub size: u32,
}

#[derive(Deserialize)]
pub struct MonoTableInfo {
    pub base: Option<Address>,
    _rows_fields: u32,
    pub size_bitfield: u32,
}

impl MonoTableInfo {
    pub fn rows(&self) -> u32 {
        self._rows_fields >> 24
    }

    pub fn row_size(&self) -> u32 {
        self._rows_fields & 0b11111111
    }
}

const MONO_TABLE_NUM: usize = 56;

#[derive(Deserialize)]
pub struct Image {
    pub ref_count: i32,
    pub storage: Option<Address>,
    pub raw_data: Option<Address>,
    pub raw_data: Option<Address>,
    pub raw_data_len: u32,
    _bitfields: [u8; 2],
    pub name: String,
    pub assembly_name: Option<String>,
    pub module_name: Option<String>,
    pub time_date_stamp: u32,
    pub version: Option<String>,
    pub md_version_major: i16,
    pub md_version_minor: i16,
    pub guid: Option<String>,
    pub image_info: Option<Address>,
    pub mempool: Option<Address>,
    pub raw_metadata: Option<Address>,
    pub heap_strings: MonoStreamHeader,
    pub heap_us: MonoStreamHeader,
    pub heap_blob: MonoStreamHeader,
    pub heap_guid: MonoStreamHeader,
    pub heap_tables: MonoStreamHeader,
    pub heap_pdb: MonoStreamHeader,
    pub tables_base: Option<Address>,
    pub referenced_tables: u64,
    pub referenced_table_rows: Option<Address>,
    pub tables: [MonoTableInfo; MONO_TABLE_NUM],
    pub references: Option<Address>,
    pub nreferences: i32,
    pub modules: Option<Address>,
    pub module_count: u32,
    pub modules_loaded: Option<Address>,
    pub files: Option<Address>,
    pub file_count: u32,
    pub aot_module: Option<Address>,
    pub aotid: [u8; 16],
    pub assembly: Option<Address>,
    pub method_cache: Option<Address>,
    pub class_cache: MonoInternalHashTable,
    pub methodref_cache: Option<Address>,
    pub field_cache: Option<Address>,
    pub typespec_cache: Option<Address>,
    pub memberref_signatures: Option<Address>,
    pub method_signatures: Option<Address>,
    pub name_cache: Option<Address>,
    pub array_cache: Option<Address>,
    pub ptr_cache: Option<Address>,
    pub szarray_cache: Option<Address>,
    pub szarray_cache_lock: MonoMutex,
    pub native_func_wrapper_cache: Option<Address>,
    pub wrapper_param_names: Option<Address>,
    pub array_accessor_cache: Option<Address>,
    pub ldfld_wrapper_cache: Option<Address>,
    pub ldflda_wrapper_cache: Option<Address>,
    pub stfld_wrapper_cache: Option<Address>,
    pub isinst_cache: Option<Address>,
    pub icall_wrapper_cache: Option<Address>,
    pub castclass_cache: Option<Address>,
    pub proxy_isinst_cache: Option<Address>,
    pub rgctx_template_hash: Option<Address>,
    pub property_hash: Option<Address>,
    pub reflection_info: Option<Address>,
    pub user_info: Option<Address>,
    // ...
}

impl Image {
    pub fn dynamic(&self) -> bool {
        has_flag(self._bitfields[0], 0b1000_0000)
    }

    pub fn ref_only(&self) -> bool {
        has_flag(self._bitfields[0], 0b0100_0000)
    }

    pub fn uncompressed_metadata(&self) -> bool {
        has_flag(self._bitfields[0], 0b0010_0000)
    }

    pub fn metadata_only(&self) -> bool {
        has_flag(self._bitfields[0], 0b0001_0000)
    }

    pub fn load_from_context(&self) -> bool {
        has_flag(self._bitfields[0], 0b0000_1000)
    }

    pub fn checked_module_cctor(&self) -> bool {
        has_flag(self._bitfields[0], 0b0000_0100)
    }

    pub fn has_module_cctor(&self) -> bool {
        has_flag(self._bitfields[0], 0b0000_0010)
    }

    pub fn idx_string_wide(&self) -> bool {
        has_flag(self._bitfields[0], 0b0000_0001)
    }

    pub fn metadata_only(&self) -> bool {
        has_flag(self._bitfields[0], 0b0001_0000)
    }

    pub fn idx_guid_wide(&self) -> bool {
        has_flag(self._bitfields[1], 0b1000_0000)
    }

    pub fn idx_blob_wide(&self) -> bool {
        has_flag(self._bitfields[1], 0b0100_0000)
    }

    pub fn core_clr_platform_code(&self) -> bool {
        has_flag(self._bitfields[1], 0b0010_0000)
    }

    pub fn minimal_delta(&self) -> bool {
        has_flag(self._bitfields[1], 0b0001_0000)
    }
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
