use std::os::raw::c_void;

use windows::Win32::System::Memory::{VirtualProtect, PAGE_PROTECTION_FLAGS, PAGE_READWRITE};

pub unsafe fn replace_function(vtable_func: *mut *mut c_void, new_func: *mut c_void, original: Option<&mut *mut c_void>) {
    if vtable_func.is_null() {
        return;
    }

    let mut old_protection = PAGE_PROTECTION_FLAGS::default();
    if VirtualProtect(vtable_func as *mut c_void, std::mem::size_of::<*mut c_void>(), PAGE_READWRITE, &mut old_protection).is_err() {
        return;
    }

    if let Some(orig) = original {
        *orig = *vtable_func;
    }

    *vtable_func = new_func;
    _= VirtualProtect(vtable_func as *mut c_void, std::mem::size_of::<*mut c_void>(), old_protection, &mut old_protection);
}

pub unsafe fn find_function(vtable: *mut *mut c_void, count: usize, opcodes: &[u8]) -> *mut *mut c_void {
    for i in 0..count {
        let func_ptr = *vtable.add(i);
        if func_ptr.is_null() {
            continue;
        }

        let func_slice = std::slice::from_raw_parts(func_ptr as *const u8, opcodes.len());
        if func_slice == opcodes {
            return vtable.add(i);
        }
    }

    std::ptr::null_mut()
}