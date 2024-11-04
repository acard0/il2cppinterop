use std::{ffi::c_void, ptr::null_mut};

use il2cppinterop_macros::Mono;

use crate::{cache, il2cpp_farproc, mono::{definitions::object::SystemObject, resolve_call}};

use super::definitions::*;

pub static mut TIME_FUNCTIONS: TimeFunctions = unsafe { std::mem::zeroed() };

pub fn initialize() {
    unsafe {
        cache::system_type_cache::initializer::add_to_be_cached_on_init(&UNITY_TIME_CLASS);

        TIME_FUNCTIONS.m_get_time = resolve_call(&UNITY_TIME_GETTIME);
        TIME_FUNCTIONS.m_get_delta_time = resolve_call(&UNITY_TIME_GETDELTATIME);
        TIME_FUNCTIONS.m_get_unscaled_time = resolve_call(&UNITY_TIME_GETUNSCALEDTIME);
        TIME_FUNCTIONS.m_get_unscaled_delta_time = resolve_call(&UNITY_TIME_GETUNSCALEDDELTATIME);
        TIME_FUNCTIONS.m_get_smooth_delta_time = resolve_call(&UNITY_TIME_GETSMOOTHDELTATIME);
        TIME_FUNCTIONS.m_get_time_scale = resolve_call(&UNITY_TIME_GETTIMESCALE);
        TIME_FUNCTIONS.m_get_frame_count = resolve_call(&UNITY_TIME_GETFRAMECOUNT);
        TIME_FUNCTIONS.m_get_realtime_since_startup = resolve_call(&UNITY_TIME_GETREALTIMESINCESTARTUP);
    }
}

#[derive(Debug, Mono)]
#[repr(C)]
pub struct Time {
    #[base]
    object: SystemObject
}

impl Time {
    pub fn get_time() -> f64 {
        unsafe { il2cpp_farproc!(fn(*mut c_void) -> f64, TIME_FUNCTIONS.m_get_time)(null_mut()) }
    }

    pub fn get_delta_time() -> f64 {
        unsafe { il2cpp_farproc!(fn(*mut c_void) -> f64, TIME_FUNCTIONS.m_get_delta_time)(null_mut()) }
    }

    pub fn get_unscaled_time() -> f64 {
        unsafe { il2cpp_farproc!(fn(*mut c_void) -> f64, TIME_FUNCTIONS.m_get_unscaled_time)(null_mut()) }
    }

    pub fn get_unscaled_delta_time() -> f64 {
        unsafe { il2cpp_farproc!(fn(*mut c_void) -> f64, TIME_FUNCTIONS.m_get_unscaled_delta_time)(null_mut()) }
    }

    pub fn get_smooth_delta_time() -> f64 {
        unsafe { il2cpp_farproc!(fn(*mut c_void) -> f64, TIME_FUNCTIONS.m_get_smooth_delta_time)(null_mut()) }
    }

    pub fn get_time_scale() -> f64 {
        unsafe { il2cpp_farproc!(fn(*mut c_void) -> f64, TIME_FUNCTIONS.m_get_time_scale)(null_mut()) }
    }

    pub fn get_frame_count() -> i32 {
        unsafe { il2cpp_farproc!(fn(*mut c_void) -> i32, TIME_FUNCTIONS.m_get_frame_count)(null_mut()) }
    }

    pub fn get_realtime_since_startup() -> f64 {
        unsafe { il2cpp_farproc!(fn(*mut c_void) -> f64, TIME_FUNCTIONS.m_get_realtime_since_startup)(null_mut()) }
    }
}

#[repr(C)]
pub struct TimeFunctions {
    m_get_time: *mut c_void,
    m_get_delta_time: *mut c_void,
    m_get_unscaled_time: *mut c_void,
    m_get_unscaled_delta_time: *mut c_void,
    m_get_smooth_delta_time: *mut c_void,
    m_get_time_scale: *mut c_void,
    m_get_frame_count: *mut c_void,
    m_get_realtime_since_startup: *mut c_void,
}
