use std::ffi::c_void;

use crate::mono::GLOBALS;

pub fn get_relative_address(rva: usize) -> *mut c_void {
    (unsafe { GLOBALS.m_base.0 } as usize + rva) as *mut c_void
}