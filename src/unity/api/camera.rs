use std::os::raw::c_void;


use crate::{cache, il2cpp::{engine::Vector3, resolve_call}, il2cpp_farproc, unity::definitions::*};

use super::game_object::CGameObject;

pub static mut CAMERA_FUNCTIONS: CameraFunctions = unsafe { std::mem::zeroed() };

pub fn initialize() {
    unsafe {
        cache::system_type_cache::initializer::add_to_be_cached_on_init(&UNITY_CAMERA_CLASS);

        CAMERA_FUNCTIONS.m_get_current = resolve_call(&UNITY_CAMERA_GETCURRENT);
        CAMERA_FUNCTIONS.m_get_main = resolve_call(&UNITY_CAMERA_GETMAIN);
        CAMERA_FUNCTIONS.m_get_depth = resolve_call(&UNITY_CAMERA_GETDEPTH);
        CAMERA_FUNCTIONS.m_set_depth = resolve_call(&UNITY_CAMERA_SETDEPTH);
        CAMERA_FUNCTIONS.m_get_field_of_view = resolve_call(&UNITY_CAMERA_GETFIELDOFVIEW);
        CAMERA_FUNCTIONS.m_set_field_of_view = resolve_call(&UNITY_CAMERA_SETFIELDOFVIEW);
        CAMERA_FUNCTIONS.m_world_to_screen = resolve_call(&UNITY_CAMERA_WORLDTOSCREEN);
    }
}

pub fn get_current() -> *mut CCamera {
    let func = il2cpp_farproc!(fn() -> *mut CCamera, CAMERA_FUNCTIONS.m_get_current);
    unsafe { func() }
}

pub fn get_main() -> *mut CCamera {
    let func = il2cpp_farproc!(fn() -> *mut CCamera, CAMERA_FUNCTIONS.m_get_main);
    unsafe { func() }
}

#[repr(C)]
pub struct CCamera {
    pub game_object: CGameObject
}

impl CCamera {
    pub fn get_depth(&self) -> f32 {
        let func = il2cpp_farproc!(fn(*mut CCamera) -> f32, CAMERA_FUNCTIONS.m_get_depth);
        unsafe { func(self as *const _ as *mut _) }
    }

    pub fn set_depth(&self, value: f32) {
        let func = il2cpp_farproc!(fn(*mut CCamera, f32) -> *mut c_void, CAMERA_FUNCTIONS.m_set_depth);
        unsafe { func(self as *const _ as *mut _, value) };
    }

    pub fn get_field_of_view(&self) -> f32 {
        let func = il2cpp_farproc!(fn(*mut CCamera) -> f32, CAMERA_FUNCTIONS.m_get_field_of_view);
        unsafe { func(self as *const _ as *mut _) }
    }

    pub fn set_field_of_view(&self, value: f32) {
        let func = il2cpp_farproc!(fn(*mut CCamera, f32) -> *mut c_void, CAMERA_FUNCTIONS.m_set_field_of_view);
        unsafe { func(self as *const _ as *mut _, value) };
    }

    pub fn world_to_screen(&self, world: &Vector3, screen: &mut Vector3, eye: i32) {
        let func = il2cpp_farproc!(fn(*mut CCamera, &Vector3, i32, &mut Vector3) -> *mut c_void, CAMERA_FUNCTIONS.m_world_to_screen);
        unsafe { func(self as *const _ as *mut _, world, eye, screen) };
    }
}

pub struct CameraFunctions {
    m_get_current: *mut std::ffi::c_void,
    m_get_main: *mut std::ffi::c_void,
    m_get_depth: *mut std::ffi::c_void,
    m_set_depth: *mut std::ffi::c_void,
    m_get_field_of_view: *mut std::ffi::c_void,
    m_set_field_of_view: *mut std::ffi::c_void,
    m_world_to_screen: *mut std::ffi::c_void,
}

#[repr(i32)]
pub enum CameraType {
    Game = 1,
    SceneView = 2,
    Preview = 4,
    VR = 8,
    Reflection = 16,
}

#[repr(i32)]
pub enum CameraEye {
    Left = 0,
    Right = 1,
    Center = 2,
}