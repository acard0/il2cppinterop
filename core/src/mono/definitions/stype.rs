
use std::{ffi::CString, mem::{transmute, zeroed}};

use getset::Getters;
use il2cppinterop_macros::Mono;

use crate::mono::{reflection::{self, meta::{Il2cppClass, Il2cppType}}, runtime};

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
        println!("getting type {}", name);

        let p_fn = reflection::class::get_method_by_name("System.Type", "GetType", 1)
            .expect("System.Type::GetType(string typeName) not found");

        println!("getting type {}, method: {}", name, p_fn.get_name());

        let p_exception: *mut *mut SystemObject = zeroed();
        let ns_system_type = CString::new(name).unwrap();
        let params = [ ns_system_type.as_bytes_with_nul().as_ptr() ];
        let boxed = runtime::runtime_invoke(p_fn, None, transmute(params.as_ptr()), p_exception);
        boxed.map(|b| transmute(b))
    }}
}