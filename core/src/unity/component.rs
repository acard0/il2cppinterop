use il2cppinterop_macros::Mono;

use crate::{cache, mono::resolve_call, il2cpp_farproc, unity::definitions::*};
use std::ffi::c_void;

use super::{game_object::UnityGameObject, object::UnityObject, transform::Transform};

pub static mut COMPONENT_FUNCTIONS: ComponentFunctions = unsafe { std::mem::zeroed() };

pub fn initialize() {
    unsafe {
        cache::system_type_cache::initializer::add_to_be_cached_on_init(&UNITY_COMPONENT_CLASS);

        COMPONENT_FUNCTIONS.m_get_game_object = resolve_call(&UNITY_COMPONENT_GETGAMEOBJECT);
        COMPONENT_FUNCTIONS.m_get_transform = resolve_call(&UNITY_COMPONENT_GETTRANSFORM);
    }
}

#[derive(Debug, Mono)]
#[repr(C)]
pub struct UnityComponent {
    #[base]
    unity_object: UnityObject
}

impl UnityComponent {
    pub fn get_game_object(&self) -> &mut UnityGameObject {
        unsafe { il2cpp_farproc!(fn(&UnityComponent) -> *mut UnityGameObject, COMPONENT_FUNCTIONS.m_get_game_object)(self).as_mut().expect("UnityComponent::GetGameObject returned nullptr") }
    }

    pub fn get_transform(&self) -> &mut Transform {
        unsafe { il2cpp_farproc!(fn(&UnityComponent) -> *mut Transform, COMPONENT_FUNCTIONS.m_get_transform)(self).as_mut().expect("UnityComponent::GetTransform returned nullptr") }
    }
}

#[repr(C)]
pub struct ComponentFunctions {
    m_get_game_object: *mut c_void,
    m_get_transform: *mut c_void,
}