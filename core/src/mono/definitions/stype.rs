
use getset::Getters;
use il2cppinterop_macros::Mono;

use crate::mono::reflection::{self, meta::{Il2cppClass, Il2cppType}};

use super::object::SystemObject;

#[derive(Debug, Mono, Getters)]
#[getset(get = "pub with_prefix")]
#[repr(C)]
pub struct SystemType {
    #[base]
    #[getset(skip)]
    object: SystemObject,
    stype: &'static mut Il2cppType
}

impl SystemType {
    /// Gets class meta represented by this member
    pub fn get_class(&self) -> &Il2cppClass {
        reflection::class::get_class_from_system_type(self)
            .unwrap()
    }

    /// Checks if represented type is a value type
    pub fn is_value_type(&self) -> bool {
        self.get_class().is_value_type()
    }
}