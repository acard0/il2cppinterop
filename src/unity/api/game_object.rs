use crate::{cache, il2cpp::{api::class, builtin::{Il2cppArray, Il2cppObject, SystemString}, resolve_call}, il2cpp_farproc, unity::definitions::*};
use std::ffi::c_void;

use super::{component::CComponent, object::CObject, transform::CTransform};

pub static mut GAME_OBJECT_FUNCTIONS: GameObjectFunctions = unsafe { std::mem::zeroed() };

pub fn initialize() {
    unsafe {
        cache::system_type_cache::initializer::add_to_be_cached_on_init(&UNITY_GAMEOBJECT_CLASS);

        GAME_OBJECT_FUNCTIONS.m_add_component = resolve_call(&UNITY_GAMEOBJECT_ADDCOMPONENT);
        GAME_OBJECT_FUNCTIONS.m_create_primitive = resolve_call(&UNITY_GAMEOBJECT_CREATEPRIMITIVE);
        GAME_OBJECT_FUNCTIONS.m_find = resolve_call(&UNITY_GAMEOBJECT_FIND);
        GAME_OBJECT_FUNCTIONS.m_find_game_objects_with_tag = resolve_call(&UNITY_GAMEOBJECT_FINDGAMEOBJECTWITHTAG);
        GAME_OBJECT_FUNCTIONS.m_get_component = resolve_call(&UNITY_GAMEOBJECT_GETCOMPONENT);
        GAME_OBJECT_FUNCTIONS.m_get_components = resolve_call(&UNITY_GAMEOBJECT_GETCOMPONENTS);
        GAME_OBJECT_FUNCTIONS.m_get_component_in_children = resolve_call(&UNITY_GAMEOBJECT_GETCOMPONENTINCHILDREN);
        GAME_OBJECT_FUNCTIONS.m_get_active = resolve_call(&UNITY_GAMEOBJECT_GETACTIVE);
        GAME_OBJECT_FUNCTIONS.m_get_layer = resolve_call(&UNITY_GAMEOBJECT_GETLAYER);
        GAME_OBJECT_FUNCTIONS.m_get_transform = resolve_call(&UNITY_GAMEOBJECT_GETTRANSFORM);
        GAME_OBJECT_FUNCTIONS.m_set_active = resolve_call(&UNITY_GAMEOBJECT_SETACTIVE);
        GAME_OBJECT_FUNCTIONS.m_set_layer = resolve_call(&UNITY_GAMEOBJECT_SETLAYER);
    }
}

pub fn find(name: &str) -> *mut CGameObject {
    let func = il2cpp_farproc!(fn(*mut SystemString) -> *mut CGameObject, GAME_OBJECT_FUNCTIONS.m_find);
    let il2cpp_string = SystemString::new(name);
    unsafe { func(il2cpp_string) }
}

pub fn find_with_tag(tag: &str) -> *mut Il2cppArray<CGameObject> {
    let func = il2cpp_farproc!(fn(*mut SystemString) -> *mut Il2cppArray<CGameObject>, GAME_OBJECT_FUNCTIONS.m_find_game_objects_with_tag);
    let il2cpp_string = SystemString::new(tag);
    unsafe { func(il2cpp_string) }
}

pub fn create_primitive(primitive_type: MEPrimitiveType) -> *mut CGameObject {
    let func = il2cpp_farproc!(fn(MEPrimitiveType) -> *mut CGameObject, GAME_OBJECT_FUNCTIONS.m_create_primitive);
    unsafe { func(primitive_type) }
}

#[repr(C)]
pub struct CGameObject {
    pub object: CObject
}

impl CGameObject {
    pub fn add_component(&self, system_type: *mut c_void) {
        let func = il2cpp_farproc!(fn(*mut CGameObject, *mut c_void) -> *mut c_void, GAME_OBJECT_FUNCTIONS.m_add_component);
        unsafe { func(self as *const _ as *mut _, system_type) };
    }

    pub fn get_component_with_name(&self, name: &str) -> *mut CComponent {
        let func = il2cpp_farproc!(fn(*mut CGameObject, *mut SystemString) -> *mut CComponent, GAME_OBJECT_FUNCTIONS.m_get_component);
        let il2cpp_string = SystemString::new(name);
        unsafe { func(self as *const _ as *mut _, il2cpp_string) }
    }

    pub fn get_component_of_type_at_index(&self, system_type: *mut Il2cppObject, index: usize) -> Option<*mut CComponent> {
        let components = self.get_components(system_type);
        if components.is_null() || index >= unsafe { (*components).m_u_max_length } {
            return None
        }

        unsafe { (*components).get_element_mut(index) }
    }

    pub fn get_component_of_stype_at_index(&self, system_type_name: &str, index: usize) -> Option<*mut CComponent> {
        let class = class::find(system_type_name);
        if class.is_null() {
            return None;
        }

        let system_type = class::get_system_type(class);
        if system_type.is_null() {
            return None;
        }

        self.get_component_of_type_at_index(system_type, index)
    }

    pub fn get_component_in_children(&self, system_type: *mut c_void, include_inactive: bool) -> *mut CComponent {
        let func = il2cpp_farproc!(fn(*mut CGameObject, *mut c_void, bool) -> *mut CComponent, GAME_OBJECT_FUNCTIONS.m_get_component_in_children);
        unsafe { func(self as *const _ as *mut _, system_type, include_inactive) }
    }

    pub fn get_components(&self, system_type: *mut Il2cppObject) -> *mut Il2cppArray<CComponent> {
        let func = il2cpp_farproc!(fn(*mut CGameObject, *mut Il2cppObject, bool, bool, bool, bool, *mut c_void) -> *mut Il2cppArray<CComponent>, GAME_OBJECT_FUNCTIONS.m_get_components);
        unsafe { func(self as *const _ as *mut _, system_type, false, false, true, false, std::ptr::null_mut()) }
    }

    pub fn get_transform(&self) -> *mut CTransform {
        let func = il2cpp_farproc!(fn(*mut CGameObject) -> *mut CTransform, GAME_OBJECT_FUNCTIONS.m_get_transform);
        unsafe { func(self as *const _ as *mut _) }
    }

    pub fn get_active(&self) -> bool {
        let func = il2cpp_farproc!(fn(*mut CGameObject) -> bool, GAME_OBJECT_FUNCTIONS.m_get_active);
        unsafe { func(self as *const _ as *mut _) }
    }

    pub fn get_layer(&self) -> u32 {
        let func = il2cpp_farproc!(fn(*mut CGameObject) -> u32, GAME_OBJECT_FUNCTIONS.m_get_layer);
        unsafe { func(self as *const _ as *mut _) }
    }

    pub fn set_active(&self, active: bool) {
        let func = il2cpp_farproc!(fn(*mut CGameObject, bool) -> *mut c_void, GAME_OBJECT_FUNCTIONS.m_set_active);
        unsafe { func(self as *const _ as *mut _, active) };
    }

    pub fn set_layer(&self, layer: u32) {
        let func = il2cpp_farproc!(fn(*mut CGameObject, u32) -> *mut c_void, GAME_OBJECT_FUNCTIONS.m_set_layer);
        unsafe { func(self as *const _ as *mut _, layer) };
    }
}

#[repr(C)]
pub struct GameObjectFunctions {
    m_add_component: *mut c_void,
    m_create_primitive: *mut c_void,
    m_find: *mut c_void,
    m_find_game_objects_with_tag: *mut c_void,
    m_get_component: *mut c_void,
    m_get_components: *mut c_void,
    m_get_component_in_children: *mut c_void,
    m_get_active: *mut c_void,
    m_get_layer: *mut c_void,
    m_get_transform: *mut c_void,
    m_set_active: *mut c_void,
    m_set_layer: *mut c_void,
}

#[repr(C)]
pub enum MEPrimitiveType {
    Sphere,
    Capsule,
    Cylinder,
    Cube,
    Plane,
    Quad,
}
