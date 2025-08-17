
use std::{ffi::{c_void, CStr, CString}, ptr::null_mut};

use meta::*;


use crate::{il2cpp_farproc, mono::{definitions::{object::ClassMemberType, stype::SystemType}, FUNCTIONS}};

use super::*;

pub fn find(class_path: &str) -> Option<&Il2cppClass> {
    unsafe {
        let mut cnt = 0;
        let assemblies = domain::get_assemblies(&mut cnt);
        if assemblies.is_null() || cnt == 0 {
            return None;
        }
        if let Some(pos) = class_path.find('+') {
            // Nested type: split namespace and type chain.
            let ns_end = class_path[..pos].rfind('.').unwrap_or(0);
            let ns = if ns_end > 0 { &class_path[..ns_end] } else { "" };
            let mut parts = class_path[ns_end + 1..].split('+');
            let root = parts.next()?;
            let mut cls = (0..cnt).find_map(|i| {
                let asm = *assemblies.add(i);
                if asm.is_null() || (*asm).m_p_image.is_null() { return None; }
                let c = get_from_name((*asm).m_p_image, ns, root);
                if c.is_null() { None } else { Some(c) }
            })?;
            for name in parts {
                cls = get_nested_classes(&*cls)
                    .into_iter()
                    .find(|nc| nc.get_class_name() == name)?;
            }
            Some(&*cls)
        } else {
            // Non-nested type.
            let (ns, name) = class_path.rsplit_once('.').unwrap_or(("", class_path));
            (0..cnt).find_map(|i| {
                let asm = *assemblies.add(i);
                if asm.is_null() || (*asm).m_p_image.is_null() { return None; }
                let c = get_from_name((*asm).m_p_image, ns, name);
                if c.is_null() { None } else { Some(c) }
            }).map(|c| &*c)
        }
    }
}

pub fn fetch_classes(module_name: &str, namespace: Option<&str>) -> Vec<&'static Il2cppClass> {
    unsafe {
        let mut cnt = 0;
        let assemblies = domain::get_assemblies(&mut cnt);
        if assemblies.is_null() || cnt == 0 {
            return Vec::new();
        }

        let image = (0..cnt)
            .map(|i| *assemblies.add(i))
            .find(|&asm| {
                !asm.is_null() && !(*asm).m_p_image.is_null() &&
                std::ffi::CStr::from_ptr((*(*asm).m_p_image).m_p_name_no_ext)
                    .to_str()
                    .unwrap_or("") == module_name
            })
            .map(|asm| (*asm).m_p_image)
            .unwrap_or(std::ptr::null_mut());

        if image.is_null() {
            return Vec::new();
        }

        let class_count = il2cpp_farproc!(fn(*mut c_void) -> usize, FUNCTIONS.m_image_get_class_count)(image as *mut c_void);
        (0..class_count)
            .filter_map(|i| {
                let class = &*il2cpp_farproc!(fn(*mut Il2cppImage, usize) -> *mut Il2cppClass, FUNCTIONS.m_image_get_class)(image, i);
                match namespace {
                    Some(ns) if class.get_class_namespace().unwrap_or("") == ns => Some(class),
                    Some(_) => None,
                    None => Some(class),
                }
            })
            .collect()
    }
}

pub fn iterate_fields(class: &Il2cppClass, iterator: &mut *mut c_void) -> Option<&'static mut Il2cppFieldInfo> {
    unsafe { il2cpp_farproc!(fn(&Il2cppClass, *mut *mut c_void) -> *mut Il2cppFieldInfo, FUNCTIONS.m_class_get_fields)(class, iterator).as_mut() }
}

pub fn iterate_methods(class: &Il2cppClass, iterator: &mut *mut c_void) -> Option<&'static mut Il2cppMethodInfo> {
    unsafe {il2cpp_farproc!(fn(&Il2cppClass, *mut *mut c_void) -> *mut Il2cppMethodInfo, FUNCTIONS.m_class_get_methods)(class, iterator).as_mut() }
}

pub fn iterate_nested_classes(class: &Il2cppClass, iterator: &mut *mut c_void) -> Option<&'static mut Il2cppClass> {
    unsafe { il2cpp_farproc!(fn(&Il2cppClass, *mut *mut c_void) -> *mut Il2cppClass, FUNCTIONS.m_class_get_nested_classes)(class, iterator).as_mut() }
}

pub fn get_fields(class: &Il2cppClass) -> Vec<&mut Il2cppFieldInfo> {
    let mut iterator: *mut c_void = unsafe { std::mem::zeroed() };
    std::iter::from_fn(|| iterate_fields(class, &mut iterator)).collect()
}

pub fn get_methods(class: &Il2cppClass) -> Vec<&mut Il2cppMethodInfo> {
    let mut iterator: *mut c_void = unsafe { std::mem::zeroed() };
    std::iter::from_fn(|| iterate_methods(class, &mut iterator)).collect()
}

pub fn get_type(class: &Il2cppClass) -> Option<&'static Il2cppType> {
    unsafe { il2cpp_farproc!(fn(&Il2cppClass) -> *mut Il2cppType, FUNCTIONS.m_class_get_type)(class).as_ref() }
}

pub fn get_system_type(class: &Il2cppClass) -> Option<&mut SystemType> {
    get_type(class).and_then(|iltype| get_system_type_from_meta(iltype))
}

pub fn get_system_type_from_meta(type_: &Il2cppType) -> Option<&mut SystemType> {
    unsafe {il2cpp_farproc!(fn(&Il2cppType) -> *mut SystemType, FUNCTIONS.m_type_get_object)(type_).as_mut() }
}

pub fn get_class_from_system_type(system_type: &SystemType) -> Option<&mut Il2cppClass> {
    unsafe { il2cpp_farproc!(fn(&Il2cppType) -> *mut Il2cppClass, FUNCTIONS.m_type_get_class)(system_type.get_stype()).as_mut() }
}

pub fn get_from_name(image: *mut Il2cppImage, namespace: &str, name: &str) -> *mut Il2cppClass {
    let sz_namespace = CString::new(namespace).unwrap();
    let sz_name = CString::new(name).unwrap();
    unsafe {il2cpp_farproc!(fn(*mut c_void, *const i8, *const i8) -> *mut Il2cppClass, FUNCTIONS.m_class_from_name)
        (image as *mut c_void, sz_namespace.as_ptr(), sz_name.as_ptr())}
}

pub fn get_system_type_by_name(class_name: &str) -> Option<&mut SystemType> {
    find(class_name).and_then(|repr| get_system_type(repr))
}

pub fn get_nested_classes(class: &Il2cppClass) -> Vec<&'static mut Il2cppClass> {
    let mut iterator: *mut c_void = unsafe { std::mem::zeroed() };
    std::iter::from_fn(|| iterate_nested_classes(class, &mut iterator)).collect()
}

pub fn get_field(class: &Il2cppClass, name: &str) -> Option<&'static mut Il2cppFieldInfo> {
    unsafe {
        let c_member_name = CString::new(name).unwrap();
        il2cpp_farproc!(fn(&Il2cppClass, *const i8) -> *mut Il2cppFieldInfo, FUNCTIONS.m_class_get_field_from_name)
            (class, c_member_name.as_ptr())
            .as_mut()
    }
}

pub fn get_field_offset(class: &Il2cppClass, name: &str) -> Option<i32> {
    get_field(class, name)
        .map(|field| field.offset)

    /*
    let mut iterator = null_mut();
    while let Some(field) = iterate_fields(class, &mut iterator).as_ref() {
        let field_name_cstr = unsafe { CStr::from_ptr(field.name) };

        if let Ok(field_name) = field_name_cstr.to_str() {
            if field_name == name {
                return Some(field.offset);
            }
        }
    }

    None
     */
}

pub fn get_field_offset_by_name(class_name: &str, name: &str) -> Option<i32> {
    find(class_name).and_then(|repr| get_field_offset(repr, name))
}

pub fn set_static_field(class: &Il2cppClass, member_name: &str, value: *mut c_void,) -> bool {
    unsafe {
        let c_member_name = CString::new(member_name).unwrap();
        let field = il2cpp_farproc!(fn(&Il2cppClass, *const i8) -> *mut Il2cppFieldInfo, FUNCTIONS.m_class_get_field_from_name)
            (class, c_member_name.as_ptr());
        if !field.is_null() {
            il2cpp_farproc!(fn(*mut Il2cppFieldInfo, *mut c_void), FUNCTIONS.m_field_static_set_value)(field, value);
            return true;
        }

        return false;
    }
}

pub fn set_static_field_by_name(class_name: &str, member_name: &str, value: *mut c_void,) -> Option<bool> {
    find(class_name).and_then(|repr| Some(set_static_field(repr, member_name, value)))
}

pub fn get_static_field_value<T>(class: &Il2cppClass, member_name: &str,) -> Option<&'static mut T> {
    unsafe {
        let c_member_name = CString::new(member_name).unwrap();
        let field = il2cpp_farproc!(fn(&Il2cppClass, *const i8) -> *mut Il2cppFieldInfo, FUNCTIONS.m_class_get_field_from_name)
            (class, c_member_name.as_ptr());

        if !field.is_null() {
            let mut value = null_mut::<T>();
            il2cpp_farproc!(fn(*mut Il2cppFieldInfo, *mut *mut T), FUNCTIONS.m_field_static_get_value)(field, &mut value);
            return value.as_mut();
        }

        None
    }
}

pub fn get_static_field_value_by_name<T>(class_name: &str, member_name: &str,) -> Option<&'static mut T> {
    find(class_name).and_then(|repr| get_static_field_value(repr, member_name))
}

pub fn get_member_type(class: &Il2cppClass, member_name: &str,) -> Option<(*mut c_void, ClassMemberType)> {
    let m_member_name = CString::new(member_name).unwrap();

    unsafe {
        let field = il2cpp_farproc!(
            fn(&Il2cppClass, *const i8) -> *mut Il2cppFieldInfo,
            FUNCTIONS.m_class_get_field_from_name
        )(class, m_member_name.as_ptr());
        if !field.is_null() {
            return Some((field as *mut c_void, ClassMemberType::Field));
        }

        let property = il2cpp_farproc!(
            fn(&Il2cppClass, *const i8) -> *mut Il2cppPropertyInfo,
            FUNCTIONS.m_class_get_property_from_name
        )(class, m_member_name.as_ptr());
        if !property.is_null() {
            return Some((property as *mut c_void, ClassMemberType::Property));
        }

        let method = il2cpp_farproc!(
            fn(&Il2cppClass, *const i8, i32) -> *mut Il2cppMethodInfo,
            FUNCTIONS.m_class_get_method_from_name
        )(class, m_member_name.as_ptr(), -1);
        if !method.is_null() {
            return Some((method as *mut c_void, ClassMemberType::Method));
        }

        None
    }
}

pub fn get_method_pointer(class: &Il2cppClass, method_name: &str, argc: i32,) -> Option<*mut c_void> {
    get_method(class, method_name, argc)
        .map(|info| info.method_pointer)
}

pub fn get_method(class: &Il2cppClass, method_name: &str, argc: i32) -> Option<&'static mut Il2cppMethodInfo> {
    let c_method_name = CString::new(method_name).unwrap();
    unsafe { il2cpp_farproc!(fn(&Il2cppClass, *const i8, i32) -> *mut Il2cppMethodInfo, FUNCTIONS.m_class_get_method_from_name)
        (class, c_method_name.as_ptr(), argc)
        .as_mut()
    }
}

pub fn get_method_by_name(class_name: &str, method_name: &str, argc: i32) -> Option<&'static mut Il2cppMethodInfo> {
    find(class_name).and_then(|repr| get_method(repr, method_name, argc))
}

pub fn get_method_pointer_by_name(class_name: &str, method_name: &str, argc: i32,) -> Option<*mut c_void> {
    find(class_name).and_then(|repr| get_method_pointer(repr, method_name, argc))
}

pub fn get_method_param_name(method_info: *mut Il2cppMethodInfo, index: u32,) -> Option<String> {
    if index >= unsafe { Into::<u32>::into((*method_info).parameter_count) } {
        return None;
    }

    let name = unsafe { il2cpp_farproc!(fn(*mut c_void, u32) -> *const i8, FUNCTIONS.m_method_get_param_name)
        (method_info as *mut c_void, index) 
    };

    match name.is_null() {
        true => None,
        _=> unsafe { CStr::from_ptr(name).to_str() }
            .ok().map(|s| s.to_string())
    }
}

pub fn get_method_param_type( method_info: *mut Il2cppMethodInfo, index: u32, ) -> *mut Il2cppType {
    if index >= unsafe { Into::<u32>::into((*method_info).parameter_count) } {
        return null_mut();
    }

    unsafe {il2cpp_farproc!(fn(*mut c_void, u32) -> *mut Il2cppType, FUNCTIONS.m_method_get_param)
        (method_info as *mut c_void, index)
    }
}

pub fn class_from_type(type_: *mut Il2cppType) -> *mut Il2cppClass {
    unsafe { il2cpp_farproc!(fn(*mut c_void) -> *mut Il2cppClass, FUNCTIONS.m_class_from_il2cpp_type)
        (type_ as *mut c_void)
    }
}

pub fn get_method_pointer_with_params(class_name: &str, method_name: &str, param_names: &[&str],) -> Option<*mut c_void> {
    unsafe {
        let class = find(class_name)?;
        let param_count = param_names.len();
        let method_name_cstr = std::ffi::CString::new(method_name).unwrap();
        let mut method_iterator = null_mut();

        while let Some(method) = iterate_methods(class, &mut method_iterator).as_ref() {
            if CStr::from_ptr(method.name as *mut i8) != method_name_cstr.as_c_str() {
                continue;
            }

            let params_match = if cfg!(feature = "unity_version_2022_3_8f1") {
                let current_param_types = method.parameters;
                (0..method.parameter_count as usize).all(|i| {
                    let param_type = *current_param_types.add(i);
                    let param_class = class_from_type(param_type);
                    (&*param_class).get_class_name() == *param_names.get(i).unwrap()
                })
            } else {
                let mut current_parameters = method.parameters;
                param_names.iter().all(|&param_name| {
                    let param_class = class_from_type(*current_parameters);
                    current_parameters = current_parameters.add(1);
                    (&*param_class).get_class_name() == param_name
                })
            };

            if params_match && param_names.len() == param_count {
                return Some(method.method_pointer);
            }
        }
        
        None
    }
}    

pub fn filter_class<'a>(classes: &'a [&'a Il2cppClass], pattern: &[&str]) -> Option<&'a Il2cppClass> {
    for next in classes.iter() {
        for &name in pattern {
            if match name.chars().next() {
                Some('~') => get_field_offset(next, &name[1..]).unwrap_or(-1) >= 0,
                Some('-') => get_method_pointer(next, &name[1..], -1).is_some(),
                _ => get_field_offset(next, name).unwrap_or(-1) >= 0 || get_method_pointer(next, name, -1).is_some(),
            } {
                return Some(*next);
            }
        }
    }

    None
}    

pub fn filter_class_to_method_pointer(classes: &[&Il2cppClass], method_name: &str, argc: i32) -> Option<*mut c_void> {
    classes.iter().find_map(|&class| get_method_pointer(class, method_name, argc))
}   