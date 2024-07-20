use crate::{cache, il2cpp::{api::class::CClass, builtin::SystemString, engine::{Quaternion, Vector3}, resolve_call}, il2cpp_farproc, unity::definitions::*};
use std::ffi::c_void;

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

#[repr(C)]
pub struct CTransform {
    pub class: CClass
}

impl CTransform {
    pub fn get_parent(&self) -> *mut CTransform {
        let func = il2cpp_farproc!(fn(*mut CTransform) -> *mut CTransform, TRANSFORM_FUNCTIONS.m_get_parent);
        unsafe { func(self as *const _ as *mut _) }
    }

    pub fn get_root(&self) -> *mut CTransform {
        let func = il2cpp_farproc!(fn(*mut CTransform) -> *mut CTransform, TRANSFORM_FUNCTIONS.m_get_root);
        unsafe { func(self as *const _ as *mut _) }
    }

    pub fn get_child(&self, index: i32) -> *mut CTransform {
        let func = il2cpp_farproc!(fn(*mut CTransform, i32) -> *mut CTransform, TRANSFORM_FUNCTIONS.m_get_child);
        unsafe { func(self as *const _ as *mut _, index) }
    }

    pub fn get_child_count(&self) -> i32 {
        let func = il2cpp_farproc!(fn(*mut CTransform) -> i32, TRANSFORM_FUNCTIONS.m_get_child_count);
        unsafe { func(self as *const _ as *mut _) }
    }

    pub fn find_child(&self, path: &str, is_active_only: bool) -> *mut CTransform {
        let func = il2cpp_farproc!(fn(*mut CTransform, *mut SystemString, bool) -> *mut CTransform, TRANSFORM_FUNCTIONS.m_find_child);
        let il2cpp_string = SystemString::new(path);
        unsafe { func(self as *const _ as *mut _, il2cpp_string, is_active_only) }
    }

    pub fn find_child_default(&self, path: &str) -> *mut CTransform {
        if path.is_empty() {
            return std::ptr::null_mut();
        }
        self.find_child(path, false)
    }

    pub fn get_position(&self) -> Vector3 {
        let func = il2cpp_farproc!(fn(*mut CTransform, &mut Vector3) -> *mut c_void, TRANSFORM_FUNCTIONS.m_get_position);
        let mut v_ret = Vector3::default();
        unsafe { func(self as *const _ as *mut _, &mut v_ret); }
        v_ret
    }

    pub fn get_rotation(&self) -> Quaternion {
        let func = il2cpp_farproc!(fn(*mut CTransform, &mut Quaternion) -> *mut c_void, TRANSFORM_FUNCTIONS.m_get_rotation);
        let mut q_ret = Quaternion::default();
        unsafe { func(self as *const _ as *mut _, &mut q_ret); }
        q_ret
    }

    pub fn get_local_position(&self) -> Vector3 {
        let func = il2cpp_farproc!(fn(*mut CTransform, &mut Vector3) -> *mut c_void, TRANSFORM_FUNCTIONS.m_get_local_position);
        let mut v_ret = Vector3::default();
        unsafe { func(self as *const _ as *mut _, &mut v_ret); }
        v_ret
    }

    pub fn get_local_scale(&self) -> Vector3 {
        let func = il2cpp_farproc!(fn(*mut CTransform, &mut Vector3) -> *mut c_void, TRANSFORM_FUNCTIONS.m_get_local_scale);
        let mut v_ret = Vector3::default();
        unsafe { func(self as *const _ as *mut _, &mut v_ret); }
        v_ret
    }

    pub fn set_position(&self, vector: Vector3) {
        let func = il2cpp_farproc!(fn(*mut CTransform, Vector3) -> *mut c_void, TRANSFORM_FUNCTIONS.m_set_position);
        unsafe { func(self as *const _ as *mut _, vector) };
    }

    pub fn set_rotation(&self, quat: Quaternion) {
        let func = il2cpp_farproc!(fn(*mut CTransform, Quaternion) -> *mut c_void, TRANSFORM_FUNCTIONS.m_set_rotation);
        unsafe { func(self as *const _ as *mut _, quat) };
    }

    pub fn set_local_position(&self, vector: Vector3) {
        let func = il2cpp_farproc!(fn(*mut CTransform, Vector3) -> *mut c_void, TRANSFORM_FUNCTIONS.m_set_local_position);
        unsafe { func(self as *const _ as *mut _, vector) };
    }

    pub fn set_local_scale(&self, vector: Vector3) {
        let func = il2cpp_farproc!(fn(*mut CTransform, Vector3) -> *mut c_void, TRANSFORM_FUNCTIONS.m_set_local_scale);
        unsafe { func(self as *const _ as *mut _, vector) };
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
