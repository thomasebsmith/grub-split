use std::collections::HashMap;

use crate::deserialize::Error as DeserializeError;
use crate::deserialize::{ArrayPtr, Deserialize, Eager, Ptr, ZeroLengthArray};
use crate::memory::{Address, MemoryReader};

use super::utils::has_flag;
use super::{Image, Object, ObjectInternals};

pub const MONO_TOKEN_TYPE_DEF: usize = 0x0200_0000;

const MONO_ROOT_DOMAIN: usize = 0;

#[derive(Debug, Deserialize)]
pub struct MonoType {
    pub data: Option<Address>,
    pub attrs: i16,
    pub typ: u8,
    _bitfields: u8,
}

#[derive(Deserialize)]
pub struct MonoClassField {
    pub typ: Ptr<MonoType>,
    pub name: String,
    pub parent: Ptr<Class>,
    pub offset: i32,
}

#[derive(Deserialize)]
pub struct MonoVTable {
    pub class: Ptr<Class>,
    pub gc_descr: Option<Address>,
    pub domain: Option<Address>,
    pub typ: Option<Address>,
    pub interface_bitmap: Option<Address>,
    pub max_interface_id: u32,
    pub rank: u8,
    pub initialized: u8,
    bitfield: u32,
    pub imt_collisions_bitmap: u32,
    pub runtime_generic_context: Option<Address>,
    pub vtable: ZeroLengthArray<Address>,
}

impl MonoVTable {
    #[must_use]
    pub fn remote(&self) -> bool {
        has_flag(self.bitfield, 0b1000_0000_0000_0000_0000_0000_0000_0000)
    }

    #[must_use]
    pub fn init_failed(&self) -> bool {
        has_flag(self.bitfield, 0b0100_0000_0000_0000_0000_0000_0000_0000)
    }

    #[must_use]
    pub fn has_static_fields(&self) -> bool {
        has_flag(self.bitfield, 0b0010_0000_0000_0000_0000_0000_0000_0000)
    }
}

#[derive(Deserialize)]
pub struct MonoClassRuntimeInfo {
    pub max_domain: u16,
    pub domain_vtables: ZeroLengthArray<Eager<Ptr<MonoVTable>>>,
}

#[derive(Deserialize)]
pub struct ClassInternals {
    pub element_class: Option<Address>,
    pub cast_class: Option<Address>,
    pub supertypes: Option<Address>,
    pub idepth: u16,
    pub rank: u8,
    pub instance_size: i32,
    _bitfields_group_1: u8,
    pub min_align: u8,
    _bitfields_group_2: [u8; 4],
    pub parent: Option<Ptr<Class>>,
    pub nested_in: Option<Ptr<Class>>,
    pub image: Option<Ptr<Image>>,
    pub name: String,
    pub name_space: String,
    pub type_token: u32,
    pub vtable_size: i32,
    pub interface_count: u16,
    pub interface_id: u32,
    pub max_interface_id: u32,
    pub interface_offsets_count: u16,
    pub interfaces_packed: Option<Address>,
    pub interface_offsets_packed: Option<Address>,
    pub interface_bitmap: Option<Address>,
    pub interfaces: Option<Address>,
    pub sizes: i32,
    pub fields: ArrayPtr<MonoClassField>,
    pub methods: Option<Address>,
    pub this_arg: MonoType,
    pub byval_arg: MonoType,
    pub gc_descr: Option<Address>,
    pub runtime_info: Eager<Option<Ptr<MonoClassRuntimeInfo>>>,
    pub vtable: Option<Address>,
    pub infrequent_data: Option<Address>,
    pub unity_user_data: Option<Address>,
    pub flags: u32,
    pub first_method_idx: u32,
    pub first_field_idx: u32,
    pub method_count: u32,
    pub field_count: u32,
    pub next_class_cache: Option<Ptr<Class>>,
}

impl ClassInternals {
    fn deserialize_vtable<M: MemoryReader>(
        &self,
        reader: &mut M,
    ) -> Result<Option<MonoVTable>, DeserializeError> {
        let Some(ref runtime_info) = self.runtime_info.value else {
            return Ok(None);
        };

        // Clippy suggests == MONO_ROOT_DOMAIN, but if MONO_ROOT_DOMAIN changes,
        // this won't work.
        #[allow(clippy::absurd_extreme_comparisons)]
        if usize::from(runtime_info.max_domain) <= MONO_ROOT_DOMAIN {
            Err(DeserializeError::InvalidStateError(format!(
                "max_domain [{}] is less than MONO_ROOT_DOMAIN [{MONO_ROOT_DOMAIN}]",
                runtime_info.max_domain,
            )))
        } else {
            Ok(Some(
                runtime_info
                    .domain_vtables
                    .nth_element(reader, MONO_ROOT_DOMAIN)?
                    .value,
            ))
        }
    }
}

pub struct Class {
    pub internals: ClassInternals,
    _vtable: Option<MonoVTable>,
    fields: HashMap<String, MonoClassField>,
    static_field_data: Option<Address>,
}

impl Deserialize for Class {
    const NUM_BYTES: usize = ClassInternals::NUM_BYTES;
    const ALIGNMENT: usize = ClassInternals::ALIGNMENT;

    fn deserialize<M: MemoryReader>(
        reader: &mut M,
        address: Address,
    ) -> Result<Self, DeserializeError> {
        let internals = ClassInternals::deserialize(reader, address)?;
        let vtable = internals.deserialize_vtable(reader).map_err(|error| {
            DeserializeError::WithContext(Box::new(error), "vtable".to_string())
        })?;

        let mut fields = HashMap::<String, MonoClassField>::new();
        // TODO: This doesn't work with container classes, generic classes, etc.
        for i in 0..internals.field_count.try_into()? {
            let field =
                internals.fields.nth_element(reader, i).map_err(|error| {
                    DeserializeError::WithContext(
                        Box::new(error),
                        format!("fields.{i}"),
                    )
                })?;
            fields.insert(field.name.clone(), field);
        }

        let static_field_data = match vtable {
            Some(ref the_vtable) /*if the_vtable.has_static_fields()*/ => {
                Some(
                    the_vtable.vtable.nth_element(
                        reader,
                        internals.vtable_size.try_into()?,
                    )?
                )
            },
            _ => None
        };

        Ok(Self {
            internals,
            _vtable: vtable,
            fields,
            static_field_data,
        })
    }
}

impl Class {
    pub fn get_static_field_address(
        &self,
        name: &str,
    ) -> Result<Address, DeserializeError> {
        let static_field_data = self.static_field_data.ok_or_else(|| {
            DeserializeError::InvalidStateError(
                "No static field data".to_string(),
            )
        })?;
        let Some(field) = self.fields.get(name) else {
            return Err(DeserializeError::InvalidStateError(format!(
                "Field \"{}\" does not exist on class \"{}\"",
                name, &self.internals.name
            )));
        };
        // TODO: perform safety checks (e.g. that field is static)
        Ok(static_field_data + field.offset.try_into()?)
    }

    pub fn get_static_field_object<'a, M: MemoryReader>(
        &'a self,
        reader: &mut M,
        name: &str,
    ) -> Result<Object<'a>, DeserializeError> {
        let address = self.get_static_field_address(name)?;
        Ok(Object {
            internals: ObjectInternals::deserialize(reader, address)?,
            class: self,
        })
    }
}
