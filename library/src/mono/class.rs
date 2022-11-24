use crate::deserialize::{Deserialize, Ptr};
use crate::memory::Address;

use super::Image;

pub const MONO_TOKEN_TYPE_DEF: usize = 0x0200_0000;

#[derive(Debug, Deserialize)]
pub struct MonoType {
    pub data: Option<Address>,
    pub attrs: i16,
    pub typ: u8,
    _bitfields: u8,
}

#[derive(Deserialize)]
pub struct Class {
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
    pub fields: Option<Address>,
    pub methods: Option<Address>,
    pub this_arg: MonoType,
    pub byval_arg: MonoType,
    pub gc_descr: Option<Address>,
    pub runtime_info: Option<Address>,
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
