use std::{ffi::{c_void, CString}, mem};

use crate::{cache, il2cpp::{builtin::SystemString, resolve_call}, il2cpp_farproc, unity::definitions::UNITY_LAYERMASK_CLASS};

pub static mut LAYER_MASK_FUNCTIONS: LayerMaskFunctions = unsafe { mem::zeroed() };

pub fn initialize() {
    unsafe {
        cache::system_type_cache::initializer::add_to_be_cached_on_init(&UNITY_LAYERMASK_CLASS);

        LAYER_MASK_FUNCTIONS.m_layer_to_name = resolve_call("UnityEngine.LayerMask::LayerToName");
        LAYER_MASK_FUNCTIONS.m_name_to_layer = resolve_call("UnityEngine.LayerMask::NameToLayer");
    }
}

pub fn layer_to_name(layer: u32) -> *mut SystemString {
    unsafe {
        let func = il2cpp_farproc!(fn(u32) -> *mut SystemString, LAYER_MASK_FUNCTIONS.m_layer_to_name);
        func(layer)
    }
}

pub fn name_to_layer(name: &str) -> u32 {
    let c_name = CString::new(name).unwrap();
    unsafe {
        let func = il2cpp_farproc!(fn(*const c_void) -> u32, LAYER_MASK_FUNCTIONS.m_name_to_layer);
        func(c_name.as_ptr() as *const c_void)
    }
}

#[repr(C)]
pub struct LayerMaskFunctions {
    pub m_layer_to_name: *mut c_void,
    pub m_name_to_layer: *mut c_void,
}