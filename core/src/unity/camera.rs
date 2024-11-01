use il2cppinterop_macros::Mono;


use crate::{cache, il2cpp_farproc, mono::resolve_call};

use super::{definitions::*, engine::Vector3, game_object::UnityGameObject};

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

pub fn get_current() -> &'static mut UnityCamera {
    unsafe { &mut *il2cpp_farproc!(fn() -> *mut UnityCamera, CAMERA_FUNCTIONS.m_get_current)() }
}

pub fn get_main() -> &'static mut UnityCamera {
    unsafe { &mut *il2cpp_farproc!(fn() -> *mut UnityCamera, CAMERA_FUNCTIONS.m_get_main)() }
}

#[derive(Debug, Mono)]
#[repr(C)]
pub struct UnityCamera {
    #[base]
    game_object: UnityGameObject
}

impl UnityCamera {
    pub fn get_depth(&self) -> f32 {
        unsafe { il2cpp_farproc!(fn(&UnityCamera) -> f32, CAMERA_FUNCTIONS.m_get_depth)(self) }
    }

    pub fn set_depth(&self, value: f32) {
        unsafe { il2cpp_farproc!(fn(&UnityCamera, f32), CAMERA_FUNCTIONS.m_set_depth)(self, value) };
    }

    pub fn get_field_of_view(&self) -> f32 {
        unsafe { il2cpp_farproc!(fn(&UnityCamera) -> f32, CAMERA_FUNCTIONS.m_get_field_of_view)(self) }
    }

    pub fn set_field_of_view(&self, value: f32) {
        unsafe { il2cpp_farproc!(fn(&UnityCamera, f32), CAMERA_FUNCTIONS.m_set_field_of_view)(self, value) };
    }

    pub fn world_to_screen(&self, world: &Vector3, eye: CameraEye, screen: &mut Vector3) {
        unsafe { il2cpp_farproc!(fn(&UnityCamera, &Vector3, i32, &mut Vector3), CAMERA_FUNCTIONS.m_world_to_screen)(self, world, eye as i32, screen) };
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

#[repr(C)]
pub enum CameraEye {
    Left = 0,
    Right = 1,
    Center = 2,
}