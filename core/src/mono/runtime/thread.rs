use std::ffi::c_void;

use crate::{il2cpp_farproc, mono::{self, reflection::domain, FUNCTIONS}, sys};

pub fn attach(domain: *mut c_void) -> *mut c_void {
    unsafe { il2cpp_farproc!(fn(*mut c_void) -> *mut c_void, FUNCTIONS.m_thread_attach)(domain) }
}

pub fn detach(thread: *mut c_void) {
    unsafe { il2cpp_farproc!(fn(*mut c_void), FUNCTIONS.m_thread_detach)(thread) }
}

pub fn current() -> *mut c_void {
    unsafe { il2cpp_farproc!(fn() -> *mut c_void, FUNCTIONS.m_thread_current)() }
}

pub fn attach_current() -> *mut c_void {
    attach(mono::reflection::domain::get())
}

pub fn spawn<F, T>(f: F)
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    sys::thread::spawn(|| { // TODO: could just use std thread
        let handle = attach(domain::get());
        f();
        detach(handle);
    });
}