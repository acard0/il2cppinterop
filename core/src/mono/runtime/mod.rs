pub mod gc;
pub mod member;
pub mod thread;

use std::{os::raw::c_void, ptr::null_mut};

pub use gc::*;
pub use member::*;
pub use thread::*;

use crate::il2cpp_farproc;

use super::{reflection::meta::Il2cppMethodInfo, FUNCTIONS};

pub fn runtime_invoke(
    method_info: &Il2cppMethodInfo, member: Option<&Il2cppObject>, params: *mut *mut c_void, exception: *mut *mut Il2cppObject
) -> Option<&'static mut Il2cppObject> {
    unsafe { il2cpp_farproc!(fn(*const Il2cppMethodInfo, *const Il2cppObject, *mut *mut c_void, *mut *mut Il2cppObject) -> *mut Il2cppObject, FUNCTIONS.m_runtime_invoke)
        (method_info as *const Il2cppMethodInfo, member.map(|m| m as *const Il2cppObject).unwrap_or(null_mut()), params, exception)
        .as_mut()
    }
}