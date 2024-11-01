use std::ffi::c_void;
use std::ptr::null_mut;

use crate::mono::runtime::*;

use super::*;

macro_rules! create_hook_mod {
    ($mod_name:ident) => {
        pub mod $mod_name {
            use super::{CallbackHook, c_void};        
            use parking_lot::Mutex;

            static CALLBACK_HOOK: Mutex<Option<CallbackHook>> = Mutex::new(None);

            pub fn add(callback: *mut c_void) {
                let mut guard = CALLBACK_HOOK.lock();
                match guard.as_mut() {
                    Some(hook) => hook.add(callback),
                    None => panic!("Attempting to add callback to uninitialized hook {}", stringify!($mod_name))
                }
            }

            pub fn initialize(vtable_pos: *mut *mut c_void) {
                let mut guard = CALLBACK_HOOK.lock();
                match guard.as_ref() {
                    None => {
                        let mut hook = CallbackHook::new(stringify!($mod_name));
                        hook.initialize(vtable_pos, __tramboline);

                        *guard = Some(hook)
                    }
                    Some(_) => {
                        panic!("Already initialized");
                    }
                }
            }

            pub fn uninitialize() {
                let mut guard = CALLBACK_HOOK.lock();
                guard.as_mut().unwrap().revert();
            }

            unsafe extern "fastcall" fn __tramboline(rcx: *mut c_void) {
                let guard = CALLBACK_HOOK.lock();
                if let Some(callback) = guard.as_ref() {
                    callback.invoke(rcx);
                }
            }
        }
    };
}

pub struct CallbackHook {
    funcs: Vec<*mut c_void>,
    vtable_pos: *mut *mut c_void,
    original: *mut c_void,
    name: String
}

unsafe impl Send for CallbackHook {}
unsafe impl Sync for CallbackHook {}

impl CallbackHook {
    pub fn new(name: &str) -> Self {
        Self {
            funcs: Vec::new(),
            vtable_pos: null_mut(),
            original: null_mut(),
            name: name.to_owned()
        }
    }

    pub fn initialize(&mut self, vtable_pos: *mut *mut c_void, tramboline: unsafe extern "fastcall" fn(*mut c_void)) {
        self.vtable_pos = vtable_pos;
        self.original = unsafe { *vtable_pos };
        unsafe { vtable::replace_function(self.vtable_pos, tramboline as _, None) };
    }

    pub fn revert(&mut self) {
        unsafe { *self.vtable_pos = self.original };
    }

    pub fn add(&mut self, func: *mut c_void) {
        self.funcs.push(func);
    }

    pub fn invoke(&self, rcx: *mut c_void) {
        self.invoke_hooks(rcx);
        self.invoke_original(rcx);
    }

    fn invoke_hooks(&self, rcx: *mut c_void) {
        for &func in self.funcs.iter() {
            let callback: extern "fastcall" fn(*mut c_void) = unsafe { std::mem::transmute(func) };
            callback(rcx);
        }
    }

    fn invoke_original(&self, rcx: *mut c_void) {
        if !self.original.is_null() {
            let original_fn: extern "fastcall" fn(*mut c_void) =
                unsafe { std::mem::transmute(self.original) };
            original_fn(rcx);
        }
    }
}

create_hook_mod!(hook_on_update);
create_hook_mod!(hook_on_late_update);

pub fn initialize() {
    unsafe {
        let il2cpp_thread = thread::attach(domain::get());
        let bhv = crate::unity::object::get_mono_behaviour();
        let mono_behaviour_vtable = bhv.get_vtable();

        if !mono_behaviour_vtable.is_null() {
            #[cfg(target_pointer_width = "64")]
            {
                hook_on_update::initialize(vtable::find_function(mono_behaviour_vtable, 99, &[0x33, 0xD2, 0xE9]));
                hook_on_late_update::initialize(vtable::find_function(mono_behaviour_vtable, 99, &[0xBA, 0x01, 0x00, 0x00, 0x00, 0xE9]));
            }
            #[cfg(target_pointer_width = "32")]
            {
                hook_on_update::initialize(vtable::find_function(mono_behaviour_vtable, 99, &[0x6A, 0x00, 0xE8]));
                hook_on_late_update::initialize(vtable::find_function(mono_behaviour_vtable, 99, &[0x6A, 0x01, 0xE8]));
            }
        } else {
            thread::detach(il2cpp_thread);    
            return;
        }

        thread::detach(il2cpp_thread);
    }
}

pub fn uninitialize() {
    hook_on_update::uninitialize();
    hook_on_late_update::uninitialize();
}