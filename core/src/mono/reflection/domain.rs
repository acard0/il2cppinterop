use std::ffi::c_void;

use crate::{il2cpp_farproc, mono::FUNCTIONS};

use super::meta::Il2cppAssembly;

pub fn get() -> *mut c_void {
    unsafe { il2cpp_farproc!(fn() -> *mut c_void, FUNCTIONS.m_domain_get)() }
}

pub fn get_assemblies(size: &mut usize) -> *mut *mut Il2cppAssembly {
    unsafe { il2cpp_farproc!(fn(*mut c_void, &mut usize) -> *mut *mut Il2cppAssembly, FUNCTIONS.m_domain_get_assemblies)
        (get(), size)
    }
}