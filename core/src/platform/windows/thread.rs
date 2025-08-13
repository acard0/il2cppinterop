use std::ffi::c_void;

use windows::Win32::Foundation::HANDLE;
use windows::Win32::Security::Authentication::Identity::RtlGenRandom;
use windows::Win32::System::Threading::{CreateThread, THREAD_CREATION_FLAGS};

pub fn random() -> usize {unsafe {
    let mut random_number: usize = 0;   
    RtlGenRandom(&mut random_number as *mut usize as *mut _, std::mem::size_of::<usize>() as u32).unwrap();
    random_number
}}

pub fn spawn<F, T>(f: F) -> HANDLE
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    let pf = Box::into_raw(Box::new(f)) as *const c_void;
    unsafe { CreateThread(None, 0, Some(thread_work_executor::<T, F>), Some(pf), THREAD_CREATION_FLAGS(0), None).unwrap() }
}

unsafe extern "system" fn thread_work_executor<Out, F: FnOnce() -> Out>(f: *mut c_void) -> u32 {
    let c = Box::from_raw(f as *mut F);
    (*c)();
    1
}