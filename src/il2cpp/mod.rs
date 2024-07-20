use std::ffi::{c_char, c_void, CString};

use windows::Win32::Foundation::HMODULE;

pub mod api;

pub mod engine;
pub mod builtin;
pub mod interop;
pub mod definitions;
pub mod utils;

pub static mut FUNCTIONS: Functions = unsafe { std::mem::zeroed() };
pub static mut GLOBALS: Globals = unsafe { std::mem::zeroed() };

pub fn resolve_call(method_name: &str) -> *mut c_void { unsafe {
    let sz_name = CString::new(method_name).unwrap();
    let call = std::mem::transmute::<_, unsafe extern "C" fn(*const c_char) -> *mut std::ffi::c_void>(FUNCTIONS.m_resolve_function)(sz_name.as_ptr());
    call
}}

#[cfg(feature = "string_encryption")]
#[macro_export]
macro_rules! IL2CPP_RStr {
    ($x:expr) => {
        crate::il2cpp::encrypt_string($x)
    };
    ($a:expr, $b:expr) => {
        crate::il2cpp::encrypt_string(&format!("{}{}", $a, $b))
    };
}

#[cfg(not(feature = "string_encryption"))]
#[macro_export]
macro_rules! IL2CPP_RStr {
    ($name:ident, $value:expr) => {
        #[allow(dead_code)]
        pub static $name: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| $value.to_string());
    };
    ($name:ident, $value1:expr, $value2:expr) => {
        #[allow(dead_code)]
        pub static $name: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
            let mut result = String::new();
            result.push_str($value1);
            result.push_str($value2);
            result
        });
    };
}


#[cfg(feature = "string_encryption")]
pub fn encrypt_string(input: String) -> String {
    format!("encrypted: {}", input)
}

pub struct Functions {
    pub m_class_from_name: *mut c_void,
    pub m_class_get_fields: *mut c_void,
    pub m_class_get_field_from_name: *mut c_void,
    pub m_class_get_methods: *mut c_void,
    pub m_class_get_method_from_name: *mut c_void,
    pub m_class_get_property_from_name: *mut c_void,
    pub m_class_get_type: *mut c_void,
    pub m_domain_get: *mut c_void,
    pub m_domain_get_assemblies: *mut c_void,
    pub m_free: *mut c_void,
    pub m_image_get_class: *mut c_void,
    pub m_image_get_class_count: *mut c_void,
    pub m_resolve_function: *mut c_void,
    pub m_string_new: *mut c_void,
    pub m_thread_attach: *mut c_void,
    pub m_thread_detach: *mut c_void,
    pub m_type_get_object: *mut c_void,
    pub m_pobject_new: *mut c_void,
    pub m_method_get_param_name: *mut c_void,
    pub m_method_get_param: *mut c_void,
    pub m_class_from_il2cpp_type: *mut c_void,
    pub m_field_static_get_value: *mut c_void,
    pub m_field_static_set_value: *mut c_void,
}

pub struct Globals {
    pub m_base: HMODULE,
    pub m_assembly: HMODULE,
    pub m_unity_player: HMODULE,
}
