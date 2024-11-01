use il2cppinterop_macros::Mono;

use std::ffi::c_void;

use crate::{cache, il2cpp_farproc, mono::{definitions::{object::SystemObject, string::SystemString}, resolve_call}};

use super::{definitions::*, engine::{Quaternion, Vector3}};

pub static mut TRANSFORM_FUNCTIONS: TransformFunctions = unsafe { std::mem::zeroed() };

pub fn initialize() {
    unsafe {
        cache::system_type_cache::initializer::add_to_be_cached_on_init(&UNITY_TRANSFORM_CLASS);

        TRANSFORM_FUNCTIONS.m_get_parent = resolve_call(&UNITY_TRANSFORM_GETPARENT);
        TRANSFORM_FUNCTIONS.m_get_root = resolve_call(&UNITY_TRANSFORM_GETROOT);
        TRANSFORM_FUNCTIONS.m_get_child = resolve_call(&UNITY_TRANSFORM_GETCHILD);
        TRANSFORM_FUNCTIONS.m_get_child_count = resolve_call(&UNITY_TRANSFORM_GETCHILDCOUNT);
        TRANSFORM_FUNCTIONS.m_find_child = resolve_call(&UNITY_TRANSFORM_FINDCHILD);
        TRANSFORM_FUNCTIONS.m_get_position = resolve_call(&UNITY_TRANSFORM_GETPOSITION);
        TRANSFORM_FUNCTIONS.m_get_rotation = resolve_call(&UNITY_TRANSFORM_GETROTATION);
        TRANSFORM_FUNCTIONS.m_get_local_position = resolve_call(&UNITY_TRANSFORM_GETLOCALPOSITION);
        TRANSFORM_FUNCTIONS.m_get_local_scale = resolve_call(&UNITY_TRANSFORM_GETLOCALSCALE);
        TRANSFORM_FUNCTIONS.m_set_position = resolve_call(&UNITY_TRANSFORM_SETPOSITION);
        TRANSFORM_FUNCTIONS.m_set_rotation = resolve_call(&UNITY_TRANSFORM_SETROTATION);
        TRANSFORM_FUNCTIONS.m_set_local_position = resolve_call(&UNITY_TRANSFORM_SETLOCALPOSITION);
        TRANSFORM_FUNCTIONS.m_set_local_scale = resolve_call(&UNITY_TRANSFORM_SETLOCALSCALE);
    }
}

#[derive(Debug, Mono)]
#[repr(C)]
pub struct Transform {
    #[base]
    object: SystemObject
}

impl Transform {
    pub fn get_parent(&self) -> &mut Transform {
        unsafe { il2cpp_farproc!(fn(&Transform) -> &mut Transform, TRANSFORM_FUNCTIONS.m_get_parent)
            (self) 
        }
    }

    pub fn get_root(&self) -> Option<&mut Transform> {
        unsafe { il2cpp_farproc!(fn(&Transform) -> *mut Transform, TRANSFORM_FUNCTIONS.m_get_root)
            (self)
            .as_mut() 
        }
    }
    
    pub fn get_child(&self, index: i32) -> Option<&mut Transform> {
        unsafe { il2cpp_farproc!(fn(&Transform, i32) -> *mut Transform, TRANSFORM_FUNCTIONS.m_get_child)
            (self, index)
            .as_mut() 
        }
    }
    
    pub fn get_child_count(&self) -> i32 {
        unsafe { il2cpp_farproc!(fn(&Transform) -> i32, TRANSFORM_FUNCTIONS.m_get_child_count)(self) }
    }
    
    pub fn find_child(&self, path: &str, is_active_only: bool) -> Option<&mut Transform> {
        match path.is_empty() {
            true => None,
            false => {
                let il2cpp_string = SystemString::new(path);
                unsafe { il2cpp_farproc!(fn(&Transform, *mut SystemString, bool) -> *mut Transform, TRANSFORM_FUNCTIONS.m_find_child)(self, il2cpp_string, is_active_only).as_mut() }
            },
        }
    }
    
    pub fn find_child_default(&self, path: &str) -> Option<&mut Transform> {
        self.find_child(path, false)
    }
    
    pub fn get_position(&self) -> Vector3 {
        let mut v_ret = Vector3::default();
        unsafe { il2cpp_farproc!(fn(&Transform, &mut Vector3), TRANSFORM_FUNCTIONS.m_get_position)(self, &mut v_ret) };
        v_ret
    }
    
    pub fn get_rotation(&self) -> Quaternion {
        let mut q_ret = Quaternion::default();
        unsafe { il2cpp_farproc!(fn(&Transform, &mut Quaternion), TRANSFORM_FUNCTIONS.m_get_rotation)(self, &mut q_ret) };
        q_ret
    }
    
    pub fn get_local_position(&self) -> Vector3 {
        let mut v_ret = Vector3::default();
        unsafe { il2cpp_farproc!(fn(&Transform, &mut Vector3), TRANSFORM_FUNCTIONS.m_get_local_position)(self, &mut v_ret) };
        v_ret
    }
    
    pub fn get_local_scale(&self) -> Vector3 {
        let mut v_ret = Vector3::default();
        unsafe { il2cpp_farproc!(fn(&Transform, &mut Vector3), TRANSFORM_FUNCTIONS.m_get_local_scale)(self, &mut v_ret) };
        v_ret
    }
    
    pub fn set_position(&self, vector: Vector3) {
        unsafe { il2cpp_farproc!(fn(&Transform, Vector3), TRANSFORM_FUNCTIONS.m_set_position)(self, vector) };
    }
    
    pub fn set_rotation(&self, quat: Quaternion) {
        unsafe { il2cpp_farproc!(fn(&Transform, Quaternion), TRANSFORM_FUNCTIONS.m_set_rotation)(self, quat) };
    }
    
    pub fn set_local_position(&self, vector: Vector3) {
        unsafe { il2cpp_farproc!(fn(&Transform, Vector3), TRANSFORM_FUNCTIONS.m_set_local_position)(self, vector) };
    }
    
    pub fn set_local_scale(&self, vector: Vector3) {
        unsafe { il2cpp_farproc!(fn(&Transform, Vector3), TRANSFORM_FUNCTIONS.m_set_local_scale)(self, vector) };
    }
}

#[repr(C)]
pub struct TransformFunctions {
    m_get_parent: *mut c_void,
    m_get_root: *mut c_void,
    m_get_child: *mut c_void,
    m_get_child_count: *mut c_void,
    m_find_child: *mut c_void,
    m_get_position: *mut c_void,
    m_get_rotation: *mut c_void,
    m_get_local_position: *mut c_void,
    m_get_local_scale: *mut c_void,
    m_set_position: *mut c_void,
    m_set_rotation: *mut c_void,
    m_set_local_position: *mut c_void,
    m_set_local_scale: *mut c_void,
}
