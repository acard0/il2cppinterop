use crate::{il2cpp_farproc, *};

use super::Il2cppObject;


pub fn suspend() {
    unsafe { il2cpp_farproc!(fn(), FUNCTIONS.m_gc_disable)() }
}

pub fn resume() {
    unsafe { il2cpp_farproc!(fn(), FUNCTIONS.m_gc_enable)() }
}

pub fn get_used_size() -> usize {
    unsafe { il2cpp_farproc!(fn() -> usize, FUNCTIONS.m_gc_get_used_size )() }
}

pub fn get_heap_size() -> usize {
    unsafe { il2cpp_farproc!(fn() -> usize, FUNCTIONS.m_gc_get_heap_size )() }
}

pub fn create_handle(object: &Il2cppObject, pinned: bool) -> *mut c_void {
    unsafe { il2cpp_farproc!(fn(&Il2cppObject, bool) -> *mut c_void, FUNCTIONS.m_gc_create_handle)(object, pinned) }
}

pub fn destroy_handle(handle: *mut c_void) {
    unsafe { il2cpp_farproc!(fn(*mut c_void), FUNCTIONS.m_gc_destroy_handle)(handle) }
}