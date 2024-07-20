use std::sync::Mutex;
use std::ffi::c_void;
use std::ptr::null_mut;

use crate::il2cpp::utils::vtable;
use crate::unity::utils::mono;

use super::domain;

pub fn initialize() {
    unsafe {
        let il2cpp_thread = super::thread::attach(domain::get());

        let bhv = mono::get_mono_behaviour();
        let mono_behaviour_vtable = *((*bhv).object.class.m_cached_ptr as *mut *mut *mut c_void);

        if !mono_behaviour_vtable.is_null() {
            #[cfg(target_pointer_width = "64")]
            {
                on_update::CALLBACK_HOOK.vfunc = vtable::find_function(mono_behaviour_vtable, 99, &[0x33, 0xD2, 0xE9]);
                on_late_update::CALLBACK_HOOK.vfunc = vtable::find_function(mono_behaviour_vtable, 99, &[0xBA, 0x01, 0x00, 0x00, 0x00, 0xE9]);
            }
            #[cfg(target_pointer_width = "32")]
            {
                on_update::CALLBACK_HOOK.vfunc = vtable::find_function(mono_behaviour_vtable, 99, &[0x6A, 0x00, 0xE8]);
                on_late_update::CALLBACK_HOOK.vfunc = vtable::find_function(mono_behaviour_vtable, 99, &[0x6A, 0x01, 0xE8]);
            }
        } else {
            super::thread::detach(il2cpp_thread);    
            return;
        }

        super::thread::detach(il2cpp_thread);

        vtable::replace_function(on_update::CALLBACK_HOOK.vfunc, on_update::perform as *mut c_void, Some(&mut on_update::CALLBACK_HOOK.original));
        vtable::replace_function(on_late_update::CALLBACK_HOOK.vfunc, on_late_update::perform as *mut c_void, Some(&mut on_late_update::CALLBACK_HOOK.original));
    }
}

pub fn uninitialize() {
    unsafe {
        vtable::replace_function(on_update::CALLBACK_HOOK.vfunc, on_update::CALLBACK_HOOK.original, None);
        vtable::replace_function(on_late_update::CALLBACK_HOOK.vfunc, on_late_update::CALLBACK_HOOK.original, None);
    }
}

pub mod on_update {
    use super::{CallbackHook, invoke_funcs};
    use std::ffi::c_void;

    pub static mut CALLBACK_HOOK: once_cell::sync::Lazy<CallbackHook> = once_cell::sync::Lazy::new(|| { CallbackHook::new() });

    pub fn add(func: *mut c_void) {
        unsafe { CALLBACK_HOOK.funcs.lock().unwrap().push(func) }
    }

    pub extern "fastcall" fn perform(rcx: *mut c_void) {
        invoke_funcs(unsafe { &CALLBACK_HOOK });
        unsafe {
            let original: extern "fastcall" fn(*mut c_void) = std::mem::transmute(CALLBACK_HOOK.original);
            original(rcx);
        }
    }
}

pub mod on_late_update {
    use super::{CallbackHook, invoke_funcs};
    use std::ffi::c_void;

    pub static mut CALLBACK_HOOK: once_cell::sync::Lazy<CallbackHook> = once_cell::sync::Lazy::new(|| { CallbackHook::new() });

    pub fn add(func: *mut c_void) {
       unsafe { CALLBACK_HOOK.funcs.lock().unwrap().push(func) };
    }

    pub extern "fastcall" fn perform(rcx: *mut c_void) {
        invoke_funcs(unsafe { &CALLBACK_HOOK });
        unsafe {
            let original: extern "fastcall" fn(*mut c_void) = std::mem::transmute(CALLBACK_HOOK.original);
            original(rcx);
        }
    }
}

fn invoke_funcs(hook: &CallbackHook) {
    let funcs = hook.funcs.lock().unwrap();
    for &func in funcs.iter() {
        let callback: extern "fastcall" fn() = unsafe { std::mem::transmute(func) };
        callback();
    }
}

pub struct CallbackHook {
    funcs: Mutex<Vec<*mut c_void>>,
    vfunc: *mut *mut c_void,
    original: *mut c_void,
}
unsafe impl Send for CallbackHook {}
unsafe impl Sync for CallbackHook {}

impl CallbackHook {
    pub fn new() -> Self {
        CallbackHook {
            funcs: Mutex::new(Vec::new()),
            vfunc: null_mut(),
            original: null_mut(),
        }
    }
}
