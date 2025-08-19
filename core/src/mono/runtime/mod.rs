pub mod gc;
pub mod member;
pub mod thread;

use std::{mem::transmute, os::raw::c_void, ptr::null_mut};

pub use gc::*;
pub use member::*;
pub use thread::*;

use crate::il2cpp_farproc;

use super::{definitions::object::SystemObject, reflection::meta::Il2cppMethodInfo, FUNCTIONS};

pub fn runtime_invoke(
    method_info: &Il2cppMethodInfo, member: Option<&SystemObject>, params: *mut *mut c_void, exception: *mut *mut SystemObject
) -> Option<&'static mut SystemObject> {
    unsafe { il2cpp_farproc!(fn(*const Il2cppMethodInfo, *const SystemObject, *mut *mut c_void, *mut *mut SystemObject) -> *mut SystemObject, FUNCTIONS.m_runtime_invoke)
        (transmute(method_info), member.map(|m| transmute(m)).unwrap_or(null_mut()), params, exception)
        .as_mut()
    }
}