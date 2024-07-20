use std::{ffi::c_void, ptr::null_mut};

use crate::{cache, il2cpp::{api::class::{self, CClass}, builtin::{Il2cppArray, Il2cppObject, SystemString}, interop::Il2cppClass, resolve_call, FUNCTIONS}, il2cpp_farproc, unity::definitions::{UNITY_OBJECT_CLASS, UNITY_OBJECT_DESTROY, UNITY_OBJECT_FINDOBJECTSOFTYPE, UNITY_OBJECT_GETNAME}};

pub static mut OBJECT_FUNCTIONS: ObjectFunctions = unsafe { std::mem::zeroed() };

pub fn initialize() {
    unsafe {
        cache::system_type_cache::initializer::add_to_be_cached_on_init(&UNITY_OBJECT_CLASS);

        OBJECT_FUNCTIONS.m_destroy = resolve_call(&UNITY_OBJECT_DESTROY);
        OBJECT_FUNCTIONS.m_find_objects_of_type = resolve_call(&UNITY_OBJECT_FINDOBJECTSOFTYPE);
        OBJECT_FUNCTIONS.m_get_name = resolve_call(&UNITY_OBJECT_GETNAME);
    }
}

pub fn new(class: *mut Il2cppClass) -> *mut Il2cppObject {
    unsafe {
        let func = il2cpp_farproc!(fn(*mut c_void) -> *mut Il2cppObject, FUNCTIONS.m_pobject_new);
        func(class as *mut c_void)
    }
}

pub fn find_objects_of_type<T>(
    system_type: *mut Il2cppObject,
    include_inactive: bool,
) -> *mut Il2cppArray<T> {
    unsafe {
        let func = il2cpp_farproc!(fn(*mut c_void, bool) -> *mut Il2cppArray<T>, OBJECT_FUNCTIONS.m_find_objects_of_type);
        func(system_type as *mut c_void, include_inactive)
    }
}

pub fn find_objects_of_type_by_name<T>(
    system_type_name: &str,
    include_inactive: bool,
) -> *mut Il2cppArray<T> {
    let class = class::find(system_type_name);
    if class.is_null() {
        return null_mut();
    }

    let system_type = class::get_system_type(class);
    if system_type.is_null() {
        log::warn!("Failed to perform typeof({})", system_type_name);
        return null_mut();
    }

    find_objects_of_type(system_type, include_inactive)
}

pub fn find_object_of_type<T>(
    system_type: *mut Il2cppObject,
    include_inactive: bool,
) -> *mut T {
    let array = find_objects_of_type::<T>(system_type, include_inactive);
    if array.is_null() || unsafe { (*array).m_u_max_length } == 0 {
        return null_mut();
    }

    for i in 0..unsafe { (*array).m_u_max_length } {
        if let Some(obj) = unsafe { (*array).get_element_mut(i) } {
            return obj;
        }
    }

    null_mut()
}

pub fn find_object_of_type_by_name<T>(
    class_path: &str,
    include_inactive: bool,
) -> *mut T {
    let class = class::find(class_path);
    if class.is_null() {
        return null_mut();
    }

    let system_type = class::get_system_type(class);
    if system_type.is_null() {
        log::warn!("Failed to perform typeof({})", class_path);
        return null_mut();
    }

    let obj = find_object_of_type::<T>(system_type, include_inactive);

    if obj.is_null() {
        log::warn!("No instance of {} were found.", class_path);
    } else {
        log::info!("Instance of {} found at location {:p}", class_path, obj);
    }

    obj
}

#[repr(C)]
pub struct CObject {
    pub class: CClass
}

impl CObject {
    pub fn destroy(&self, time_delay: f32) {
        unsafe {
            let func = il2cpp_farproc!(fn(*mut c_void, f32) -> *mut c_void, OBJECT_FUNCTIONS.m_destroy);
            func(self as *const _ as *mut _, time_delay);
        }
    }

    pub fn get_name(&self) -> *mut SystemString {
        unsafe {
            let func = il2cpp_farproc!(fn(*mut c_void) -> *mut SystemString, OBJECT_FUNCTIONS.m_get_name);
            func(self as *const _ as *mut _)
        }
    }
}

#[repr(C)]
pub struct ObjectFunctions {
    pub m_destroy: *mut c_void,
    pub m_find_objects_of_type: *mut c_void,
    pub m_get_name: *mut c_void,
}