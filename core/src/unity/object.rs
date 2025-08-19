use std::ffi::c_void;

use il2cppinterop_macros::Mono;

use crate::{cache, il2cpp_farproc, mono::{definitions::{array::Il2cppArray, object::SystemObject, string::SystemString}, reflection::class, resolve_call}};

use super::{component::UnityComponent, definitions::*, game_object::UnityGameObject};

pub static mut OBJECT_FUNCTIONS: UnityObjectFunctions = unsafe { std::mem::zeroed() };

pub fn initialize() {
    unsafe {
        cache::system_type_cache::initializer::add_to_be_cached_on_init(&UNITY_OBJECT_CLASS);

        OBJECT_FUNCTIONS.m_destroy = resolve_call(&UNITY_OBJECT_DESTROY);
        OBJECT_FUNCTIONS.m_find_objects_of_type = resolve_call(&UNITY_OBJECT_FINDOBJECTSOFTYPE);
        OBJECT_FUNCTIONS.m_get_name = resolve_call(&UNITY_OBJECT_GETNAME);
    }
}

pub fn find_objects_of_type<T>(system_type: &SystemObject, include_inactive: bool,) -> Option<&mut Il2cppArray<T>> {
    unsafe { il2cpp_farproc!(fn(&SystemObject, bool) -> *mut Il2cppArray<T>, OBJECT_FUNCTIONS.m_find_objects_of_type)
        (system_type, include_inactive)
        .as_mut() 
    }
}

pub fn find_objects_of_type_by_name<T>(system_type_name: &str, include_inactive: bool,) -> Option<&mut Il2cppArray<T>> {
    class::find(system_type_name)
        .and_then(|repr| class::get_system_type(repr))
        .and_then(|system_type| find_objects_of_type(system_type, include_inactive))
}

pub fn find_object_of_type<T>(system_type: &SystemObject, include_inactive: bool,) -> Option<&T> {
    find_objects_of_type::<T>(system_type, include_inactive)
        .and_then(|candidates: &mut Il2cppArray<T>| {
            for candidate in candidates.into_iter() {
                return Some(candidate);
            }

            None
        })
}

pub fn find_object_of_type_by_name<T>(class_path: &str, include_inactive: bool,) -> Option<&T> {
    class::find(class_path)
        .and_then(|repr| class::get_system_type(repr))
        .and_then(|system_type| find_object_of_type(system_type, include_inactive))
}

pub fn get_mono_behaviour() -> &'static mut UnityComponent {
    let objects = find_objects_of_type_by_name::<UnityGameObject>(&UNITY_GAMEOBJECT_CLASS, false)
        .expect("Failed to get UNITY_GAMEOBJECT_CLASS instances");

    if objects.is_empty() {
        panic!("Used find_objects_of_type_by_name::<CGameObject> to get UNITY_GAMEOBJECT_CLASS instances but got empty collection from il2cpp runtime");
    }

    objects.into_iter().find_map(|game_object| {
        game_object.get_component_of_stype_at_index(&UNITY_MONOBEHAVIOUR_CLASS, 0)
    }).expect("Used get_component_of_stype_at_index to get UnityEngine.MonoBehaviour from an UnityGameObject but got none.")
}


#[derive(Debug, Mono)]
#[repr(C)]
pub struct UnityObject {
    #[base]
    system_object: SystemObject,
    cached_ptr: isize,
}

impl UnityObject {
    pub fn get_vtable(&self) -> *mut *mut c_void {
        unsafe { *(self.cached_ptr as *mut *mut *mut c_void) }
    }
    
    pub fn get_name(&self) -> String {
        unsafe { il2cpp_farproc!(fn(&UnityObject) -> *mut SystemString, OBJECT_FUNCTIONS.m_get_name)
            (self)
            .as_ref()
            .map(|and| and.to_string())
            .unwrap_or(String::new())
        }
    }

    pub fn destroy(&self, time_delay: f32) {
        unsafe { il2cpp_farproc!(fn(&UnityObject, f32), OBJECT_FUNCTIONS.m_destroy)
            (self, time_delay) 
        }
    }
}

#[repr(C)]
pub struct UnityObjectFunctions {
    pub m_destroy: *mut c_void,
    pub m_find_objects_of_type: *mut c_void,
    pub m_get_name: *mut c_void,
}