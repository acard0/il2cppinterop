use std::{ffi::{c_void, CString}, mem::transmute, ptr::null_mut};

use utils::get_method_pointer;

use crate::{il2cpp::{builtin::Il2cppObject, interop::{Il2cppAssembly, Il2cppClass, Il2cppFieldInfo, Il2cppImage, Il2cppMethodInfo, Il2cppPropertyInfo, Il2cppType}, FUNCTIONS}, il2cpp_farproc};

use super::domain;

pub fn get_fields(class: *mut Il2cppClass, iterator: &mut *mut c_void) -> *mut Il2cppFieldInfo {
    unsafe {
        let func = il2cpp_farproc!(fn(*mut c_void, *mut *mut c_void) -> *mut Il2cppFieldInfo, FUNCTIONS.m_class_get_fields);
        func(class as *mut c_void, iterator)
    }
}

pub fn fetch_fields(
    class: *mut Il2cppClass,
    vector: &mut Vec<*mut Il2cppFieldInfo>,
    field_iterator: *mut c_void,
) {
    vector.clear();

    let mut iterator = field_iterator;
    loop {
        let field = get_fields(class, &mut iterator);
        if field.is_null() {
            break;
        }
        vector.push(field);
    }
}

pub fn get_methods(class: *mut Il2cppClass, iterator: &mut *mut c_void) -> *mut Il2cppMethodInfo {
    unsafe {
        let func = il2cpp_farproc!(fn(*mut c_void, *mut *mut c_void) -> *mut Il2cppMethodInfo, FUNCTIONS.m_class_get_methods);
        func(class as *mut c_void, iterator)
    }
}

pub fn fetch_methods(
    class: *mut Il2cppClass,
    vector: &mut Vec<*mut Il2cppMethodInfo>,
    method_iterator: *mut c_void,
) {
    vector.clear();

    let mut iterator = method_iterator;
    loop {
        let method = get_methods(class, &mut iterator);
        if method.is_null() {
            break;
        }
        vector.push(method);
    }
}

pub fn get_type(class: *mut Il2cppClass) -> *mut Il2cppType {
    unsafe {
        let func = il2cpp_farproc!(fn(*mut c_void) -> *mut Il2cppType, FUNCTIONS.m_class_get_type);
        func(class as *mut c_void)
    }
}

pub fn get_system_type(class: *mut Il2cppClass) -> *mut Il2cppObject {
    unsafe {
        let func = il2cpp_farproc!(fn(*mut Il2cppType) -> *mut Il2cppObject,FUNCTIONS.m_type_get_object);
        func(get_type(class))
    }
}

pub fn get_from_name(image: *mut Il2cppImage, namespace: &str, name: &str) -> *mut Il2cppClass {
    let sz_namespace = CString::new(namespace).unwrap();
    let sz_name = CString::new(name).unwrap();
    unsafe {
        let func = il2cpp_farproc!(fn(*mut c_void, *const i8, *const i8) -> *mut Il2cppClass, FUNCTIONS.m_class_from_name);
        func(image as *mut c_void, sz_namespace.as_ptr(), sz_name.as_ptr())
    }
}

pub fn find(class_path: &str) -> *mut Il2cppClass {
    unsafe {
        let mut assemblies_count: usize = 0;
        let assemblies = domain::get_assemblies(&mut assemblies_count);
        if assemblies.is_null() || assemblies_count == 0 {
            log::warn!("domain::get_assemblies returned nullptr");
            return null_mut();
        }

        let (s_namespace, s_class) = match class_path.rfind('.') {
            Some(pos) => (&class_path[..pos], &class_path[pos+1..]),
            _ => ("", class_path),
        };

        for i in 0..assemblies_count {
            let assembly = *assemblies.add(i);
            if assembly.is_null() || (*assembly).m_p_image.is_null() {
                log::warn!("encountered nullptr while iterating through assembly collection retreived via domain::get_assemblies");
                continue;
            }

            let class = get_from_name((*assembly).m_p_image, s_namespace, s_class);
            if !class.is_null() {
                return class;
            }
        }
    }
    null_mut()
}

#[repr(C)]
pub struct CClass {
    pub m_object: Il2cppObject,
    pub m_cached_ptr: *mut c_void,
}

impl CClass {
    pub fn get_fields(&self, iterator: &mut *mut c_void) -> *mut Il2cppFieldInfo {
        get_fields(self.m_object.m_p_class, iterator)
    }

    pub fn fetch_fields(
        &self,
        vector: &mut Vec<*mut Il2cppFieldInfo>,
        field_iterator: *mut c_void,
    ) {
        fetch_fields(self.m_object.m_p_class, vector, field_iterator)
    }

    pub fn get_methods(&self, iterator: &mut *mut c_void) -> *mut Il2cppMethodInfo {
        get_methods(self.m_object.m_p_class, iterator)
    }

    pub fn fetch_methods(
        &self,
        vector: &mut Vec<*mut Il2cppMethodInfo>,
        method_iterator: *mut c_void,
    ) {
        fetch_methods(self.m_object.m_p_class, vector, method_iterator)
    }

    pub fn get_method_pointer(&self, method_name: &str, args: i32) -> *mut c_void {
        get_method_pointer(self.m_object.m_p_class, method_name, args)
    }

    pub fn get_prop_type(&self, prop_type: &str) -> ClassPropType {
        let c_prop_type = CString::new(prop_type).unwrap();
        unsafe {
            let get_field_func = il2cpp_farproc!(fn(*mut c_void, *const i8) -> *mut Il2cppFieldInfo, FUNCTIONS.m_class_get_field_from_name);
            let field = get_field_func(self.m_object.m_p_class as *mut c_void, c_prop_type.as_ptr());
            if !field.is_null() {
                return ClassPropType::Field;
            }

            let get_property_func = il2cpp_farproc!(fn(*mut c_void, *const i8) -> *mut Il2cppPropertyInfo, FUNCTIONS.m_class_get_property_from_name);
            let property = get_property_func(self.m_object.m_p_class as *mut c_void, c_prop_type.as_ptr());
            if !property.is_null() {
                return ClassPropType::Property;
            }

            let get_method_func = il2cpp_farproc!(fn(*mut c_void, *const i8, i32) -> *mut Il2cppMethodInfo, FUNCTIONS.m_class_get_method_from_name);
            let method = get_method_func(self.m_object.m_p_class as *mut c_void, c_prop_type.as_ptr(), -1);
            if !method.is_null() {
                return ClassPropType::Method;
            }

            ClassPropType::Unknown
        }
    }

    pub fn call_method<TReturn, TArgs>(&self, method: *mut c_void, args: TArgs) -> TReturn {
        unsafe {
            let func = il2cpp_farproc!(fn(*mut c_void, TArgs) -> TReturn, method);
            func(self as *const _ as *mut _, args)
        }
    }

    pub fn call_method_by_name<TReturn, TArgs>(&self, method_name: &str, args: TArgs) -> TReturn {
        self.call_method(self.get_method_pointer(method_name, -1), args)
    }

    pub fn call_method_safe<TReturn, TArgs>(&self, method: *mut c_void, args: TArgs) -> Option<TReturn> {
        if method.is_null() {
            return None;
        }

        Some(self.call_method(method, args))
    }

    pub fn call_method_safe_by_name<TReturn, TArgs>(&self, method_name: &str, args: TArgs) -> Option<TReturn> {
        self.call_method_safe(self.get_method_pointer(method_name, -1), args)
    }

    pub fn get_property_value<T>(&self, property_name: &str) -> Option<T> {
        let c_property_name = CString::new(property_name).unwrap();
        unsafe {
            let func = il2cpp_farproc!(fn(*mut c_void, *const i8) -> *mut Il2cppPropertyInfo, FUNCTIONS.m_class_get_property_from_name);
            let property = func(self.m_object.m_p_class as *mut c_void, c_property_name.as_ptr());

            if !property.is_null() && !(*property).m_p_get.is_null() {
                let get_func = il2cpp_farproc!(fn(*mut c_void) -> T, (*property).m_p_get);
                Some(get_func(self as *const _ as *mut _))
            } else {
                None
            }
        }
    }

    pub fn set_property_value<T>(&self, property_name: &str, value: T) {
        let c_property_name = CString::new(property_name).unwrap();
        unsafe {
            let func = il2cpp_farproc!(fn(*mut c_void, *const i8) -> *mut Il2cppPropertyInfo, FUNCTIONS.m_class_get_property_from_name);
            let property = func(self.m_object.m_p_class as *mut c_void, c_property_name.as_ptr());
            if !property.is_null() && !(*property).m_p_get.is_null() {
                let set_func = il2cpp_farproc!(fn(*mut c_void, T) -> *mut c_void, (*property).m_p_get);
                set_func(self as *const _ as *mut _, value);
            }
        }
    }

    pub fn get_member_value<T>(&self, offset: isize) -> &T {
        unsafe { &*((self as *const _ as *const u8).offset(offset) as *const T) }
    }

    pub fn set_member_value<T>(&self, offset: isize, value: T) {
        unsafe {
            let ptr = (self as *const _ as *mut u8).offset(offset) as *mut T;
            *ptr = value;
        }
    }

    pub fn get_member_value_by_field<T>(&self, field: *mut Il2cppFieldInfo) -> Option<&T> {
        if field.is_null() || unsafe { (*field).m_i_offset } < 0 {
            None
        } else {
            Some(self.get_member_value(unsafe { (*field).m_i_offset } as isize))
        }
    }

    pub fn set_member_value_by_field<T>(&self, field: *mut Il2cppFieldInfo, value: T) {
        if !field.is_null() && unsafe { (*field).m_i_offset } >= 0 {
            self.set_member_value(unsafe { (*field).m_i_offset } as isize, value);
        }
    }

    pub fn get_member_value_by_name<T>(&self, member_name: &str) -> Option<&T> {
        let c_member_name = CString::new(member_name).unwrap();
        unsafe {
            let func = il2cpp_farproc!(fn(*mut c_void, *const i8) -> *mut Il2cppFieldInfo, FUNCTIONS.m_class_get_field_from_name);
            let field = func(self.m_object.m_p_class as *mut c_void, c_member_name.as_ptr());

            if !field.is_null() {
                return match (*field).m_i_offset >= 0 {
                    true => Some(self.get_member_value((*field).m_i_offset as isize)),
                    false => None,
                }
            } else {
                return self.get_property_value(member_name);
            }
        }
    }

    pub fn set_member_value_by_name<T>(&self, member_name: &str, value: T) {
        let c_member_name = CString::new(member_name).unwrap();
        unsafe {
            let func = il2cpp_farproc!(fn(*mut c_void, *const i8) -> *mut Il2cppFieldInfo, FUNCTIONS.m_class_get_field_from_name);
            let field = func(self.m_object.m_p_class as *mut c_void, c_member_name.as_ptr());
            if !field.is_null() {
                if (*field).m_i_offset >= 0 {
                    self.set_member_value((*field).m_i_offset as isize, value);
                }
            } else {
                self.set_property_value(member_name, value);
            }
        }
    }

    pub fn get_obscured_via_offset<T: Default + Copy>(&self, offset: isize) -> Option<T> {
        if offset >= 0 {
            unsafe {
                match std::mem::size_of::<T>() {
                    size if size == std::mem::size_of::<f64>() => {
                        let key = self.get_member_value::<usize>(offset);
                        let value = self.get_member_value::<usize>(offset + std::mem::size_of::<usize>() as isize) ^ key;
                        Some(*(value as *const T))
                    }
                    size if size == std::mem::size_of::<i32>() => {
                        let key = self.get_member_value::<i32>(offset);
                        let value = self.get_member_value::<i32>(offset + std::mem::size_of::<i32>() as isize) ^ key;
                        Some(*(value as *const T))
                    }
                    size if size == std::mem::size_of::<bool>() => {
                        let key = self.get_member_value::<u8>(offset);
                        let value = self.get_member_value::<i32>(offset + std::mem::size_of::<u8>() as isize) ^ *key as i32;
                        Some(*(value as *const T))
                    }
                    _ => None,
                }
            }
        } else {
            Default::default()
        }
    }

    pub fn get_obscured_value<T: Default + Copy>(&self, member_name: &str) -> Option<T> {
        let c_member_name = CString::new(member_name).unwrap();
        unsafe {
            let func = il2cpp_farproc!(fn(*mut c_void, *const i8) -> *mut Il2cppFieldInfo, FUNCTIONS.m_class_get_field_from_name);
            let field = func(self.m_object.m_p_class as *mut c_void, c_member_name.as_ptr());
            self.get_obscured_via_offset(if !field.is_null() { (*field).m_i_offset as isize } else { -1 })
        }
    }

    pub fn set_obscured_via_offset<T: Copy>(&self, offset: isize, value: T) {
        if offset >= 0 {
            unsafe {
                match std::mem::size_of::<T>() {
                    size if size == std::mem::size_of::<f64>() => {
                        let key = self.get_member_value::<usize>(offset);
                        let transmuted_value = *std::mem::transmute::<*const T, *const usize>(&value) ^ key;
                        self.set_member_value(offset + std::mem::size_of::<usize>() as isize, transmuted_value);
                    }
                    size if size == std::mem::size_of::<i32>() => {
                        let key = self.get_member_value::<i32>(offset);
                        let transmuted_value = *std::mem::transmute::<*const T, *const i32>(&value) ^ key;
                        self.set_member_value(offset + std::mem::size_of::<i32>() as isize, transmuted_value);
                    }
                    size if size == std::mem::size_of::<bool>() => {
                        let key = self.get_member_value::<u8>(offset);
                        let transmuted_value = *std::mem::transmute::<*const T, *const i32>(&value) ^ *key as i32;
                        self.set_member_value(offset + std::mem::size_of::<u8>() as isize, transmuted_value);
                    }
                    _ => (),
                }
            }
        }
    }

    pub fn set_obscured_value<T: Copy>(&self, member_name: &str, value: T) {
        let c_member_name = CString::new(member_name).unwrap();
        unsafe {
            let func = il2cpp_farproc!(fn(*mut c_void, *const i8) -> *mut Il2cppFieldInfo, FUNCTIONS.m_class_get_field_from_name);
            let field = func(self.m_object.m_p_class as *mut c_void, c_member_name.as_ptr());
            if !field.is_null() {
                self.set_obscured_via_offset((*field).m_i_offset as isize, value);
            }
        }
    }
}

pub enum ClassPropType {
    Unknown,
    Field,
    Property,
    Method,
}

pub fn get_system_type_by_name(class_name: &str) -> *mut Il2cppObject {
    let class = find(class_name);
    if class.is_null() {
        return null_mut();
    }
    get_system_type(class)
}

pub fn fetch_classes(
    vector: &mut Vec<*mut Il2cppClass>,
    module_name: &str,
    namespace: Option<&str>,
) {
    vector.clear();

    unsafe {
        let mut assemblies_count: usize = 0;
        let assemblies: *mut *mut Il2cppAssembly = transmute(domain::get_assemblies(&mut assemblies_count));
        if assemblies.is_null() || assemblies_count == 0 {
            return;
        }

        let mut image = null_mut();
        for i in 0..assemblies_count {
            let assembly = *assemblies.add(i);
            if assembly.is_null() || (*assembly).m_p_image.is_null() {
                continue;
            }
            if std::ffi::CStr::from_ptr((*(*assembly).m_p_image).m_p_name_no_ext)
                .to_str()
                .unwrap()
                == module_name
            {
                image = (*assembly).m_p_image;
                break;
            }
        }

        if !image.is_null() {
            let class_count = {
                let func = il2cpp_farproc!(fn(*mut c_void) -> usize, FUNCTIONS.m_image_get_class_count);
                func(image as *mut c_void)
            };
            for i in 0..class_count {
                let class = {
                    let func = il2cpp_farproc!(fn(*mut c_void, usize) -> *mut Il2cppClass, FUNCTIONS.m_image_get_class);
                    func(image as *mut c_void, i)
                };
                if let Some(ns) = namespace {
                    if ns.is_empty() && !(*class).m_p_namespace.is_null() {
                        continue;
                    }
                    if std::ffi::CStr::from_ptr((*class).m_p_namespace)
                        .to_str()
                        .unwrap()
                        != ns
                    {
                        continue;
                    }
                }
                vector.push(class);
            }
        }
    }
}

pub mod utils {
    use std::ffi::CStr;

    use super::*;

    pub fn get_field_offset(class: *mut Il2cppClass, name: &str) -> i32 {
        let mut iterator = null_mut();
        loop {
            let field = get_fields(class, &mut iterator);
            if field.is_null() {
                break;
            }
            if unsafe { std::ffi::CStr::from_ptr((*field).m_p_name).to_str().unwrap() } == name {
                return (unsafe { &*field }).m_i_offset;
            }
        }
        -1
    }

    pub fn get_field_offset_by_name(class_name: &str, name: &str) -> i32 {
        let class = find(class_name);
        if !class.is_null() {
            return get_field_offset(class, name);
        }
        -1
    }

    pub fn set_static_field(
        class: *mut Il2cppClass,
        member_name: &str,
        value: *mut c_void,
    ) {
        let c_member_name = CString::new(member_name).unwrap();
        let field = unsafe {
            let func = il2cpp_farproc!(fn(*mut c_void, *const i8) -> *mut Il2cppFieldInfo, FUNCTIONS.m_class_get_field_from_name);
            func(class as *mut c_void, c_member_name.as_ptr())
        };
        if !field.is_null() {
            unsafe {
                let func = il2cpp_farproc!(fn(*mut Il2cppFieldInfo, *mut c_void) -> *mut c_void, FUNCTIONS.m_field_static_set_value);
                func(field, value);
            }
        }
    }

    pub fn set_static_field_by_name(
        class_name: &str,
        member_name: &str,
        value: *mut c_void,
    ) {
        let class = find(class_name);
        if !class.is_null() {
            set_static_field(class, member_name, value);
        }
    }

    pub fn get_static_field(
        class: *mut Il2cppClass,
        member_name: &str,
    ) -> *mut c_void {
        let c_member_name = CString::new(member_name).unwrap();
        let field = unsafe {
            let func = il2cpp_farproc!(fn(*mut c_void, *const i8) -> *mut Il2cppFieldInfo, FUNCTIONS.m_class_get_field_from_name);
            func(class as *mut c_void, c_member_name.as_ptr())
        };
        if !field.is_null() {
            let mut value = null_mut();
            unsafe {
                let func = il2cpp_farproc!(fn(*mut Il2cppFieldInfo, *mut *mut c_void) -> *mut c_void, FUNCTIONS.m_field_static_get_value);
                func(field, &mut value);
            }
            return value;
        }
        null_mut()
    }

    pub fn get_static_field_by_name(
        class_name: &str,
        member_name: &str,
    ) -> *mut c_void {
        let class = find(class_name);
        if !class.is_null() {
            return get_static_field(class, member_name);
        }
        null_mut()
    }

    pub fn get_method_pointer(
        class: *mut Il2cppClass,
        method_name: &str,
        args: i32,
    ) -> *mut c_void {
        let c_method_name = CString::new(method_name).unwrap();
        let method = unsafe {
            let func = il2cpp_farproc!(fn(*mut c_void, *const i8, i32) -> *mut Il2cppMethodInfo, FUNCTIONS.m_class_get_method_from_name);
            func(class as *mut c_void, c_method_name.as_ptr(), args)
        };
        if method.is_null() {
            eprintln!(
                "[DEBUG] Failed to find method {}, inside class.",
                method_name
            );
            return null_mut();
        }
        println!(
            "[DEBUG] Found method ptr of {} at {:p}.",
            method_name, (unsafe {&*method}).m_p_method_pointer
        );
        (unsafe { &*method }).m_p_method_pointer
    }

    pub fn get_method_pointer_by_name(
        class_name: &str,
        method_name: &str,
        args: i32,
    ) -> *mut c_void {
        let class = find(class_name);
        if !class.is_null() {
            return get_method_pointer(class, method_name, args);
        }
        null_mut()
    }

    pub fn get_method_param_name(
        method_info: *mut Il2cppMethodInfo,
        index: u32,
    ) -> Option<String> {
        if index >= unsafe { (*method_info).m_u_args_count.into() } {
            return None;
        }

        let name = unsafe {
            let func = il2cpp_farproc!(fn(*mut c_void, u32) -> *const i8, FUNCTIONS.m_method_get_param_name);
            func(method_info as *mut c_void, index)
        };

        if name.is_null() {
            None
        } else {
            Some(unsafe { CString::from_raw(name as *mut i8) }
                .to_str()
                .unwrap()
                .to_string())
        }
    }

    pub fn get_method_param_type(
        method_info: *mut Il2cppMethodInfo,
        index: u32,
    ) -> *mut Il2cppType {
        if index >= unsafe { (*method_info).m_u_args_count.into() } {
            return null_mut();
        }

        unsafe {
            let func = il2cpp_farproc!(fn(*mut c_void, u32) -> *mut Il2cppType, FUNCTIONS.m_method_get_param);
            func(method_info as *mut c_void, index)
        }
    }

    pub fn class_from_type(type_: *mut Il2cppType) -> *mut Il2cppClass {
        unsafe {
            let func = il2cpp_farproc!(fn(*mut c_void) -> *mut Il2cppClass, FUNCTIONS.m_class_from_il2cpp_type);
            func(type_ as *mut c_void)
        }
    }

    pub fn get_method_pointer_with_params(
        class_name: &str,
        method_name: &str,
        param_names: &[&str],
    ) -> *mut c_void {
        let class = find(class_name);
        if class.is_null() {
            return null_mut();
        }

        let param_count = param_names.len();

        let mut method_iterator = null_mut();
        loop {
            let method = get_methods(class, &mut method_iterator);
            if method.is_null() {
                break;
            }

            if unsafe { CStr::from_ptr((*method).m_p_name as *mut i8) }.to_str().unwrap() != method_name
            {
                continue;
            }

            #[cfg(unity_version_2022_3_8f1)] {
                let current_param_types = (unsafe { *method }).m_p_parameters;
                for i in 0..(unsafe { *method }).m_u_args_count as usize {
                    let param_type = (unsafe { *current_param_types.wrapping_add(i) }).m_p_parameter_type;
                    let param_class = class_from_type(param_type);
                    if unsafe { CStr::from_ptr((*param_class).m_p_name as *mut i8) }.to_str().unwrap()
                        != param_names[i]
                    {
                        break;
                    }
                    if i + 1 == param_count {
                        return (unsafe { *method }).m_p_method_pointer;
                    }
                }
            }
            #[cfg(not(unity_version_2022_3_8f1))]
            {
                let mut current_parameters = (unsafe { &*method }).m_p_parameters;
                for i in 0..param_count {
                    if unsafe { CStr::from_ptr((*current_parameters).m_p_name as *mut i8) }.to_str().unwrap() != param_names[i]
                    {
                        break;
                    }
                    
                    current_parameters = unsafe { current_parameters.add(1) };
                    if i + 1 == param_count {
                        return (unsafe { &*method }).m_p_method_pointer;
                    }
                }
            }
        }
        null_mut()
    }

    pub fn filter_class(
        classes: &Vec<*mut Il2cppClass>,
        names: &[&str],
        found_count: usize,
    ) -> *mut Il2cppClass {
        let names_count = names.len();
        let found_count = if found_count <= 0 || found_count > names_count { names_count } else { found_count };

        for &class in classes {
            if class.is_null() {
                continue;
            }

            let mut found_now = 0;
            for &name in names {
                let found_in_class = if name.starts_with('~') {
                    get_field_offset(class, &name[1..]) >= 0
                } else if name.starts_with('-') {
                    !get_method_pointer(class, &name[1..], -1).is_null()
                } else {
                    get_field_offset(class, name) >= 0
                        || !get_method_pointer(class, name, -1).is_null()
                };

                if found_in_class {
                    found_now += 1;
                }

                if found_now == found_count {
                    return class;
                }
            }
        }
        null_mut()
    }

    pub fn filter_class_to_method_pointer(
        classes: &Vec<*mut Il2cppClass>,
        method_name: &str,
        args: i32,
    ) -> *mut c_void {
        for &class in classes {
            if class.is_null() {
                continue;
            }

            let method_pointer = get_method_pointer(class, method_name, args);
            if !method_pointer.is_null() {
                return method_pointer;
            }
        }
        null_mut()
    }
}