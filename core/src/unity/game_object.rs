use il2cppinterop_macros::Mono;
use std::ffi::c_void;

use crate::{cache, il2cpp_farproc, mono::{definitions::{array::Il2cppArray, string::SystemString, stype::SystemType}, reflection::class, resolve_call}};

use super::{component::UnityComponent, definitions::*, object::UnityObject, transform::Transform};

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

pub fn find(game_object_name: &str) -> Option<&'static mut UnityGameObject> {
    let il2cpp_string = SystemString::new(game_object_name);
    unsafe { il2cpp_farproc!(fn(*mut SystemString) -> *mut UnityGameObject, GAME_OBJECT_FUNCTIONS.m_find)
        (il2cpp_string)
        .as_mut() 
    }
}

pub fn find_with_tag(game_object_tag: &str) -> Option<&mut Il2cppArray<UnityGameObject>> {
    let il2cpp_string = SystemString::new(game_object_tag);
    unsafe { (il2cpp_farproc!(fn(*mut SystemString) -> *mut Il2cppArray<UnityGameObject>, GAME_OBJECT_FUNCTIONS.m_find_game_objects_with_tag)
        (il2cpp_string))
        .as_mut() 
    }
}

pub fn create_primitive(primitive_type: MEPrimitiveType) -> Option<&'static mut UnityGameObject> {
    unsafe { il2cpp_farproc!(fn(MEPrimitiveType) -> *mut UnityGameObject, GAME_OBJECT_FUNCTIONS.m_create_primitive)
        (primitive_type)
        .as_mut() 
    }
}

#[derive(Debug, Mono)]
#[repr(C)]
pub struct UnityGameObject {
    #[base]
    unity_object: UnityObject
}

impl UnityGameObject {
    pub fn add_component(&self, system_type: &SystemType) {
        unsafe { il2cpp_farproc!(fn(&UnityGameObject, &SystemType), GAME_OBJECT_FUNCTIONS.m_add_component)
            (self, system_type) 
        };
    }
    
    pub fn get_component_with_name(&self, name: &str) -> Option<&mut UnityComponent> {
        let il2cpp_string = SystemString::new(name);
        unsafe { il2cpp_farproc!(fn(&UnityGameObject, *mut SystemString) -> *mut UnityComponent, GAME_OBJECT_FUNCTIONS.m_get_component)
            (self, il2cpp_string)
            .as_mut() 
        }
    }
    
    pub fn get_component_of_type_at_index(&self, system_type: &SystemType, index: usize) -> Option<&mut UnityComponent> {
        self.get_components(system_type)
            .get_mut(index)
    }
    
    pub fn get_component_of_stype_at_index(&self, system_type_name: &str, index: usize) -> Option<&mut UnityComponent> {
        class::find(system_type_name)
            .and_then(|repr| class::get_system_type(repr))
            .and_then(|system_type| self.get_component_of_type_at_index(system_type, index))
    }
    
    pub fn get_component_in_children(&self, system_type: &SystemType, include_inactive: bool) -> Option<&mut UnityComponent> {
        unsafe { il2cpp_farproc!(fn(&UnityGameObject, &SystemType, bool) -> *mut UnityComponent, GAME_OBJECT_FUNCTIONS.m_get_component_in_children)
            (self, system_type, include_inactive)
            .as_mut() 
        }
    }
    
    pub fn get_components(&self, system_type: &SystemType) -> &mut Il2cppArray<UnityComponent> {
        //void GetComponentsInternal(System.Type* type,bool useSearchTypeAsArrayReturnType,bool recursive,bool includeInactive,bool reverse,void* resultList);
        unsafe { (il2cpp_farproc!(fn(&UnityGameObject, &SystemType, bool, bool, bool, bool, *mut c_void) -> *mut Il2cppArray<UnityComponent>, GAME_OBJECT_FUNCTIONS.m_get_components)
            (self, system_type, false, false, true, false, std::ptr::null_mut()) as *mut Il2cppArray<UnityComponent>)
            .as_mut()
            .expect("UnityGameObject::GetComponents returned null ptr")
        }
    }
    
    pub fn get_transform(&self) -> &mut Transform {
        unsafe { il2cpp_farproc!(fn(&UnityGameObject) -> *mut Transform, GAME_OBJECT_FUNCTIONS.m_get_transform)
            (self)
            .as_mut()
            .expect("UnityGameObject::GetTransform returned null ptr") 
        }
    }
    
    pub fn get_active(&self) -> bool {
        unsafe { il2cpp_farproc!(fn(&UnityGameObject) -> bool, GAME_OBJECT_FUNCTIONS.m_get_active)
            (self) 
        }
    }
    
    pub fn get_layer(&self) -> u32 {
        unsafe { il2cpp_farproc!(fn(&UnityGameObject) -> u32, GAME_OBJECT_FUNCTIONS.m_get_layer)
            (self) 
        }
    }
    
    pub fn set_active(&self, active: bool) {
        unsafe { il2cpp_farproc!(fn(&UnityGameObject, bool), GAME_OBJECT_FUNCTIONS.m_set_active)
            (self, active) 
        };
    }
    
    pub fn set_layer(&self, layer: u32) {
        unsafe { il2cpp_farproc!(fn(&UnityGameObject, u32), GAME_OBJECT_FUNCTIONS.m_set_layer)
            (self, layer) 
        };
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