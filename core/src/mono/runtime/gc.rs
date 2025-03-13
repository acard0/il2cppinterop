use crate::{il2cpp_farproc, *};

use super::Il2cppObject;


pub fn suspend() {
    unsafe { il2cpp_farproc!(fn(), FUNCTIONS.m_gc_disable)() }
}

pub fn resume() {
    unsafe { il2cpp_farproc!(fn(), FUNCTIONS.m_gc_enable)() }
}

pub fn is_disabled() -> bool {
    unsafe { il2cpp_farproc!(fn() -> bool, FUNCTIONS.m_gc_is_disabled)() }
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

pub fn create_weakref(object: &Il2cppObject, track: bool) -> *mut c_void {
    unsafe { il2cpp_farproc!(fn(&Il2cppObject, bool) -> *mut c_void, FUNCTIONS.m_gc_create_weakref)(object, track) }
}

pub fn get_weakref_target(handle: *mut c_void) -> Option<&'static mut Il2cppObject> {
    unsafe { il2cpp_farproc!(fn(*mut c_void) -> *mut Il2cppObject, FUNCTIONS.m_gc_weakref_get_target)(handle).as_mut() }    
}

pub fn collect(max_generations: i32) {
    unsafe { il2cpp_farproc!(fn(i32), FUNCTIONS.m_gc_collect)(max_generations) }
}

pub fn collect_a_little() {
    unsafe { il2cpp_farproc!(fn(), FUNCTIONS.m_gc_collect_a_little)() }
}