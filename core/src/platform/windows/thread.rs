use std::collections::VecDeque;
use std::ffi::c_void;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use windows::Win32::Foundation::{CloseHandle, HANDLE, WAIT_FAILED, WAIT_TIMEOUT};
use windows::Win32::Security::Authentication::Identity::RtlGenRandom;
use windows::Win32::System::Threading::{CreateEventA, CreateEventW, CreateThread, ResetEvent, SetEvent, WaitForSingleObject, INFINITE, THREAD_CREATION_FLAGS};

use crate::error::Error;

#[derive(Clone)]
pub struct Notify {
    event: HANDLE,
    waiters: Arc<Mutex<usize>>,
}

impl Notify {
    pub fn new() -> Result<Arc<Self>, Error> {
        let event = unsafe { CreateEventA(None, true, false, None) }?;
        Ok(Arc::new(Notify {
            event,
            waiters: Arc::new(Mutex::new(0)),
        }))
    }

    pub fn notify(&self){
        _= unsafe { SetEvent(self.event) };
    }

    pub fn wait(self: &Arc<Self>) -> Result<(), Error> {
        {
            let lock = &*self.waiters;
            let mut count = lock.lock().unwrap();
            *count += 1;
        }

        unsafe { WaitForSingleObject(self.event, INFINITE) };

        {
            let lock = &*self.waiters;
            let mut count = lock.lock().unwrap();
            *count -= 1;
            if *count == 0 {
                unsafe { ResetEvent(self.event) }?;
            }
        }

        Ok(())
    }

    pub fn wait_timeout(self: &Arc<Self>, timeout: Duration) -> bool {
        {
            let lock = &*self.waiters;
            let mut count = lock.lock().unwrap();
            *count += 1;
        }

        let result = unsafe { WaitForSingleObject(self.event, timeout.as_millis() as u32) };

        {
            let lock = &*self.waiters;
            let mut count = lock.lock().unwrap();
            *count -= 1;
            if *count == 0 {
                _= unsafe { ResetEvent(self.event) };
            }
        }

        result != WAIT_TIMEOUT
    }
}

impl Drop for Notify {
    fn drop(&mut self) {
        _= unsafe { CloseHandle(self.event) };
    }
}

pub struct WeakEvent<T> {
    queue: Mutex<VecDeque<T>>,
    event: HANDLE,
}

impl<T> WeakEvent<T> {
    pub fn new() -> Arc<Self> { unsafe {
        let event = CreateEventW(None, true, false, None).unwrap();
        if event.is_invalid() {
            panic!("Failed to create event");
        }

        Arc::new(Self {
            queue: Mutex::new(VecDeque::new()),
            event,
        })
    }}

    pub fn send(&self, msg: T) -> Result<(), Box<dyn std::error::Error>> {
        {
            let mut queue = self.queue.lock().unwrap();
            queue.push_back(msg);
        }

        unsafe {
            if let Err(err) = SetEvent(self.event) {
                return Err(format!("Failed to signal queue event. {:?}", err).into());
            }
        }

        Ok(())
    }

    pub fn receive(&self) -> Result<T, Box<dyn std::error::Error>> {
        loop {
            {
                let mut queue = self.queue.lock().unwrap();
                if let Some(msg) = queue.pop_front() {
                    return Ok(msg);
                }
            }

            unsafe {
                let result = WaitForSingleObject(self.event, INFINITE);
                if result == WAIT_FAILED {
                    return Err("WaitForSingleObject failed.".into());
                }
            }
        }
    }
}

impl<T> Drop for WeakEvent<T> {
    fn drop(&mut self) {
        unsafe {
            _= CloseHandle(self.event);
        }
    }
}

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