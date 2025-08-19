
use std::{mem::{transmute, zeroed}, ptr::null_mut};

use getset::Getters;
use il2cppinterop_macros::Mono;

use crate::mono::{definitions::string::SystemString, reflection::{self, meta::{Il2cppClass, Il2cppType}}, runtime};

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
    pub fn get_class(&self) -> &mut Il2cppClass {
        reflection::class::get_class_from_system_type(self)
            .unwrap()
    }

    /// Checks if represented type is a value type
    pub fn is_value_type(&self) -> bool {
        self.get_class().is_value_type()
    }

    pub fn get_type(name: &str) -> Option<&mut SystemType> {unsafe {
        let p_fn = reflection::class::get_method_by_name("System.Type", "GetType", 1)
            .expect("System.Type::GetType(string typeName) not found");

        let p_exception: *mut *mut SystemObject = zeroed();
        let params = [ SystemString::new(name) ];
        let boxed = runtime::runtime_invoke(p_fn, None, transmute(params.as_ptr()), p_exception)
            .map(|m| m as *mut SystemObject).unwrap_or(null_mut());

        transmute(boxed)
    }}
}