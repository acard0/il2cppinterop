use std::ffi::c_void;

use crate::{il2cpp::{interop::Il2cppAssembly, FUNCTIONS}, il2cpp_farproc};


pub fn get() -> *mut c_void {
    unsafe {
        let func = il2cpp_farproc!(fn() -> *mut c_void, FUNCTIONS.m_domain_get);
        func()
    }
}

pub fn get_assemblies(size: &mut usize) -> *mut *mut Il2cppAssembly {
    unsafe {
        let func = il2cpp_farproc!(fn(*mut c_void, &mut usize) -> *mut *mut Il2cppAssembly, FUNCTIONS.m_domain_get_assemblies);
        func(get(), size)
    }
}