use crate::deserialize::{Deserialize, Eager, Ptr};
use crate::memory::Address;

use super::{Class, MonoVTable};

#[derive(Deserialize)]
pub struct ObjectInternals {
    pub vtable: Eager<Ptr<MonoVTable>>,
    pub synchronization: Option<Address>,
}

pub struct Object<'a> {
    pub internals: ObjectInternals,
    pub class: &'a Class,
}
