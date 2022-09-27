use std::io;

use crate::deserialize::Error as DeserializeError;
use crate::deserialize::{Deserialize, Eager, Ptr};
use crate::memory::{
    Address, MemoryLocator, MemoryReader, MemorySearcher,
    VariableLengthAddressRange,
};

use super::{GHashTable, MonoInternalHashTable};

const MONO_LIBRARY_NAME: &str = "libmonobdwgc-2.0.dylib";
const LOADED_IMAGES_OFFSET: usize = 0x0016_d638 + 0x0018_e978 + 0x10;

const SIZE_OF_MONO_MUTEX: usize = 64;

type GHashTablePtr<K, V> = Eager<Ptr<GHashTable<K, V>>>;

type MonoMutex = [u8; SIZE_OF_MONO_MUTEX];
type MonoWrapperCaches = [Option<Address>; 21];

#[must_use]
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
    rows_fields: u32,
    pub size_bitfield: u32,
}

impl MonoTableInfo {
    #[must_use]
    pub fn rows(&self) -> u32 {
        self.rows_fields >> 24
    }

    #[must_use]
    pub fn row_size(&self) -> u32 {
        self.rows_fields & 0b1111_1111
    }
}

const MONO_TABLE_NUM: usize = 56;

#[derive(Deserialize)]
pub struct Image {
    pub ref_count: i32,
    pub raw_data_handle: Option<Address>,
    pub raw_data: Option<Address>,
    pub raw_data_len: u32,
    bitfields: [u8; 2],
    pub name: String,
    pub assembly_name: Option<String>,
    pub module_name: Option<String>,
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
    pub name_cache: GHashTablePtr<String, GHashTablePtr<String, u64>>,
    pub array_cache: Option<Address>,
    pub ptr_cache: Option<Address>,
    pub szarray_cache: Option<Address>,
    pub szarray_cache_lock: MonoMutex,
    pub native_func_wrapper_cache: Option<Address>,
    pub runtime_invoke_vcall_cache: Option<Address>,
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
    pub dll_map: Option<Address>,
    pub interface_bitset: Option<Address>,
    pub reflection_info_unregister_classes: Option<Address>,
    pub image_sets: Option<Address>,
    pub wrapper_caches: MonoWrapperCaches,
    pub var_cache_fast: Option<Address>,
    pub mvar_cache_fast: Option<Address>,
    pub var_cache_slow: Option<Address>,
    pub mvar_cache_slow: Option<Address>,
    pub var_cache_constrained: Option<Address>,
    pub mvar_cache_constrained: Option<Address>,
    pub pinvoke_scopes: Option<Address>,
    pub pinvoke_scope_filenames: Option<Address>,
    pub loader: Option<Address>,
    pub anonymous_generic_class_container: Option<Address>,
    pub anonymous_generic_method_container: Option<Address>,
    pub weak_fields_inited: bool,
    pub weak_field_indexes: Option<Address>,
    pub lock: MonoMutex,
}

impl Image {
    #[must_use]
    pub fn raw_buffer_used(&self) -> bool {
        has_flag(self.bitfields[0], 0b1000_0000)
    }

    #[must_use]
    pub fn raw_data_allocated(&self) -> bool {
        has_flag(self.bitfields[0], 0b0100_0000)
    }

    #[must_use]
    pub fn fileio_used(&self) -> bool {
        has_flag(self.bitfields[0], 0b0010_0000)
    }

    #[must_use]
    pub fn dynamic(&self) -> bool {
        has_flag(self.bitfields[0], 0b0001_0000)
    }

    #[must_use]
    pub fn ref_only(&self) -> bool {
        has_flag(self.bitfields[0], 0b0000_1000)
    }

    #[must_use]
    pub fn uncompressed_metadata(&self) -> bool {
        has_flag(self.bitfields[0], 0b0000_0100)
    }

    #[must_use]
    pub fn metadata_only(&self) -> bool {
        has_flag(self.bitfields[0], 0b0000_0010)
    }

    #[must_use]
    pub fn load_from_context(&self) -> bool {
        has_flag(self.bitfields[0], 0b0000_0001)
    }

    #[must_use]
    pub fn checked_module_cctor(&self) -> bool {
        has_flag(self.bitfields[1], 0b1000_0000)
    }

    #[must_use]
    pub fn has_module_cctor(&self) -> bool {
        has_flag(self.bitfields[1], 0b0100_0000)
    }

    #[must_use]
    pub fn idx_string_wide(&self) -> bool {
        has_flag(self.bitfields[1], 0b0010_0000)
    }

    #[must_use]
    pub fn idx_guid_wide(&self) -> bool {
        has_flag(self.bitfields[1], 0b0001_0000)
    }

    #[must_use]
    pub fn idx_blob_wide(&self) -> bool {
        has_flag(self.bitfields[1], 0b0000_1000)
    }

    #[must_use]
    pub fn core_clr_platform_code(&self) -> bool {
        has_flag(self.bitfields[1], 0b0000_0100)
    }
}

type ImageHashTable = GHashTablePtr<String, Eager<Ptr<Image>>>;

const MONO_TEXT_SECTION_PATTERN: [u8; 48] = [
    0xcf, 0xfa, 0xed, 0xfe, 0x07, 0x00, 0x00, 0x01, 0x03, 0x00, 0x00, 0x00,
    0x06, 0x00, 0x00, 0x00, 0x13, 0x00, 0x00, 0x00, 0x48, 0x08, 0x00, 0x00,
    0x85, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00,
    0x78, 0x02, 0x00, 0x00, 0x5f, 0x5f, 0x54, 0x45, 0x58, 0x54, 0x00, 0x00,
];

const SEARCHER: MemorySearcher =
    MemorySearcher::new(&MONO_TEXT_SECTION_PATTERN);

const MAX_OFFSET_BYTES: usize = 1024 * 4096;

pub struct LoadedImages {
    loaded_images_by_name: ImageHashTable,
}

impl LoadedImages {
    pub fn new<L: MemoryLocator, M: MemoryReader>(
        locator: &mut L,
        reader: &mut M,
    ) -> Result<Self, DeserializeError> {
        let base_addr = locator.locate(MONO_LIBRARY_NAME)?;
        println!("base addr is {}", base_addr);
        let text_addr = SEARCHER
            .search(
                reader,
                VariableLengthAddressRange {
                    start: base_addr,
                    num_bytes: MAX_OFFSET_BYTES,
                },
            )?
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::Other, "TEXT signature not found")
            })?;
        println!("TEXT addr is {}", text_addr);

        let addr = text_addr + LOADED_IMAGES_OFFSET;
        println!("loaded images addr is {}", addr);
        Ok(Self {
            loaded_images_by_name: ImageHashTable::deserialize(reader, addr)?,
        })
    }

    #[must_use]
    pub fn get_image(&self, name: &str) -> Option<&Image> {
        let eager_ptr = self.loaded_images_by_name.value.get(name)?;
        Some(&eager_ptr.value)
    }
}
