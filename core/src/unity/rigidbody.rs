use std::{ffi::c_void, mem};

use crate::{cache, mono::resolve_call, il2cpp_farproc, unity::definitions::*};

use super::engine::Vector3;

pub static mut RIGIDBODY_FUNCTIONS: RigidbodyFunctions = unsafe { mem::zeroed() };

pub fn initialize() {
    unsafe {
        cache::system_type_cache::initializer::add_to_be_cached_on_init(&UNITY_RIGIDBODY_CLASS);

        RIGIDBODY_FUNCTIONS.m_get_detect_collisions = resolve_call(&UNITY_RIGIDBODY_GETDETECTCOLLISIONS);
        RIGIDBODY_FUNCTIONS.m_get_velocity = resolve_call(&UNITY_RIGIDBODY_GETVELOCITY);
        RIGIDBODY_FUNCTIONS.m_set_detect_collisions = resolve_call(&UNITY_RIGIDBODY_SETDETECTCOLLISIONS);
        RIGIDBODY_FUNCTIONS.m_set_velocity = resolve_call(&UNITY_RIGIDBODY_SETVELOCITY);
    }
}

#[repr(C)]
pub struct CRigidbody {
    pub m_object: *mut c_void,
}

impl CRigidbody {
    pub fn get_detect_collisions(&self) -> bool {
        unsafe {
            let func = il2cpp_farproc!(fn(*mut c_void) -> bool, RIGIDBODY_FUNCTIONS.m_get_detect_collisions);
            func(self as *const _ as *mut _)
        }
    }

    pub fn set_detect_collisions(&self, detect: bool) {
        unsafe {
            let func = il2cpp_farproc!(fn(*mut c_void, bool), RIGIDBODY_FUNCTIONS.m_set_detect_collisions);
            func(self as *const _ as *mut _, detect);
        }
    }

    pub fn get_velocity(&self) -> Vector3 {
        let mut v_ret = Vector3::default();
        unsafe {
            let func = il2cpp_farproc!(fn(*mut c_void, &mut Vector3), RIGIDBODY_FUNCTIONS.m_get_velocity);
            func(self as *const _ as *mut _, &mut v_ret);
        }
        v_ret
    }

    pub fn set_velocity(&self, vector: Vector3) {
        unsafe {
            let func = il2cpp_farproc!(fn(*mut c_void, Vector3), RIGIDBODY_FUNCTIONS.m_set_velocity);
            func(self as *const _ as *mut _, vector);
        }
    }
}

#[repr(C)]
pub struct RigidbodyFunctions {
    pub m_get_detect_collisions: *mut c_void,
    pub m_get_velocity: *mut c_void,
    pub m_set_detect_collisions: *mut c_void,
    pub m_set_velocity: *mut c_void,
}