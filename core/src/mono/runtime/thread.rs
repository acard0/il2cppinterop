use std::{ffi::c_void, mem::transmute};

use windows::Win32::{Foundation::{CloseHandle, HANDLE}, System::Threading::{CreateThread, THREAD_CREATION_FLAGS}};

use crate::{il2cpp_farproc, mono::{reflection::domain, FUNCTIONS}};

pub fn attach(domain: *mut c_void) -> *mut c_void {
    unsafe { il2cpp_farproc!(fn(*mut c_void) -> *mut c_void, FUNCTIONS.m_thread_attach)(domain) }
}

pub fn detach(thread: *mut c_void) {
    unsafe { il2cpp_farproc!(fn(*mut c_void), FUNCTIONS.m_thread_detach)(thread) }
}

pub fn current() -> *mut c_void {
    unsafe { il2cpp_farproc!(fn() -> *mut c_void, FUNCTIONS.m_thread_current)() }
}

#[derive(Clone)]
pub struct CThread {
    on_start: *mut c_void,
    on_end: *mut c_void,
}

impl CThread {
    pub fn new(on_start: *mut c_void, on_end: *mut c_void) -> Self {
        if on_start.is_null() {
            panic!("IL2CPP::CThread - on_start is nullptr");
        }

        let thread = CThread { on_start, on_end };
        let m_thread = Box::into_raw(Box::new(thread.clone()));

        unsafe {
            let handle: HANDLE = CreateThread(
                None,
                0,
                Some(handler),
                Some(m_thread as *mut c_void),
                THREAD_CREATION_FLAGS(0),
                None,
            ).unwrap();

            if !handle.is_invalid() {
                _= CloseHandle(handle);
            }
        }

        thread
    }
}

unsafe extern "system" fn handler(reserved: *mut c_void) -> u32 {
    unsafe {
        let il2cpp_thread = attach(domain::get());

        let thread = Box::from_raw(reserved as *mut CThread);
        let on_start = il2cpp_farproc!(fn(), thread.on_start);
        let on_end: Option<unsafe extern "C" fn()> = if thread.on_end.is_null() {
            None
        } else {
            Some(transmute(thread.on_end))
        };

        on_start();
        if let Some(end) = on_end {
            end();
        }

        detach(il2cpp_thread);
    }
    0
}