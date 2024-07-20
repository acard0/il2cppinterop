use crate::{cache, il2cpp::resolve_call, il2cpp_farproc, unity::definitions::*};
use std::ffi::c_void;

use super::{game_object::CGameObject, object::CObject, transform::CTransform};

pub static mut COMPONENT_FUNCTIONS: ComponentFunctions = unsafe { std::mem::zeroed() };

pub fn initialize() {
    unsafe {
        cache::system_type_cache::initializer::add_to_be_cached_on_init(&UNITY_COMPONENT_CLASS);

        COMPONENT_FUNCTIONS.m_get_game_object = resolve_call(&UNITY_COMPONENT_GETGAMEOBJECT);
        COMPONENT_FUNCTIONS.m_get_transform = resolve_call(&UNITY_COMPONENT_GETTRANSFORM);
    }
}

#[repr(C)]
pub struct CComponent {
    pub object: CObject
}

impl CComponent {
    pub fn get_game_object(&self) -> *mut CGameObject {
        let func = il2cpp_farproc!(fn(*mut CComponent) -> *mut CGameObject, COMPONENT_FUNCTIONS.m_get_game_object);
        unsafe { func(self as *const _ as *mut _) }
    }

    pub fn get_transform(&self) -> *mut CTransform {
        let func = il2cpp_farproc!(fn(*mut CComponent) -> *mut CTransform, COMPONENT_FUNCTIONS.m_get_transform);
        unsafe { func(self as *const _ as *mut _) }
    }
}

#[repr(C)]
pub struct ComponentFunctions {
    m_get_game_object: *mut c_void,
    m_get_transform: *mut c_void,
}