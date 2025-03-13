use std::{os::raw::c_void, ptr::null_mut};
use std::ffi::CString;

use il2cppinterop_macros::Mono;
use meta::*;

use crate::{il2cpp_farproc, mono::{reflection::*, runtime::*, FUNCTIONS}, platform::mem::{self, AsArrayOfBytePattern}};

pub fn new<T>(class: &Il2cppClass) -> Option<&mut T> {
    unsafe { il2cpp_farproc!(fn(&Il2cppClass) -> *mut T, FUNCTIONS.m_object_new)
        (class)
        .as_mut()
    }
}

pub fn new_from_namespace<T>(namespace: &str) -> Option<&mut T> {
    class::find(namespace)
        .and_then(|class| new(class))
}

/// Finds an object of type `T` within a given `Il2cppClass` that matches the specified predicate.
pub fn find<T, F>(repr: &Il2cppClass, predicate: F) -> mem::Result<Option<*mut T>>
where
    T: Sized,
    F: Fn(*mut c_void) -> bool,
{
    let pattern = {
        // ensure this gets dropped before call to aob_query
        // otherwise it would result additional entry in query result
        let location = repr as *const _ as usize;
        location.as_array_of_byte_pattern()
    };

    let candidates: Vec<usize> = mem::aob_query(&pattern, false, false, true, false, None)?;
    let result = candidates.into_iter()
        .find(|&candidate| predicate(candidate as *mut c_void))
        .map(|candidate| candidate as *mut T);

    Ok(result)
}  

#[derive(Debug, Mono)]
#[repr(C)]
pub struct SystemObject {
    #[base]
    object: Il2cppObject,
}

impl SystemObject {
    pub fn iterate_fields(&self, iterator: &mut *mut c_void) -> Option<&mut Il2cppFieldInfo> {
        class::iterate_fields(self, iterator)
    }

    pub fn get_fields(&self,) -> Vec<&mut Il2cppFieldInfo> {
        class::get_fields(self)
    }

    pub fn iterate_methods(&self, iterator: &mut *mut c_void) -> Option<&mut Il2cppMethodInfo> {
        class::iterate_methods(self, iterator)
    }

    pub fn get_methods(&self) -> Vec<&mut Il2cppMethodInfo>{
        class::get_methods(self)
    }

    pub fn get_method_pointer(&self, method_name: &str, argc: i32) -> Option<*mut c_void> {
        class::get_method_pointer(self, method_name, argc)
    }

    pub fn get_member_type(&self, member_name: &str) -> ClassMemberType {
        class::get_member_type(&self, member_name)
            .map(|t| t.1)
            .unwrap_or(ClassMemberType::Unknown)
    }    

    pub fn call_method<TReturn, TArg1>(&self, method: *mut c_void, arg1: TArg1) -> TReturn {
        unsafe { il2cpp_farproc!(fn(*mut c_void, ...) -> TReturn, method)
                (self as *const _ as *mut _, arg1)
        }
    }

    pub fn get_member_value_unsafe<T: Sized>(&self, offset: isize) -> *mut T {
        unsafe { (self as *const _ as *const u8).offset(offset) as *mut T }
    }     

    pub fn get_member_value<T: Sized>(&self, offset: isize) -> Option<&'static mut T> {
        unsafe { self.get_member_value_unsafe::<T>(offset).as_mut() }
    }

    pub fn set_member_value<T: Sized>(&self, offset: isize, value: T) {
        unsafe {
            let ptr = (self as *const _ as *mut u8).offset(offset) as *mut T;
            *ptr = value;
        }
    }
 
    pub fn get_property_value_unsafe<T: Sized>(&self, property_name: &str) -> *mut T {
        let c_property_name = CString::new(property_name).unwrap();
        unsafe {
            let func = il2cpp_farproc!(fn(&Il2cppClass, *const i8) -> *mut Il2cppPropertyInfo, FUNCTIONS.m_class_get_property_from_name);
            let property = func(self, c_property_name.as_ptr());

            if !property.is_null() && !(*property).get.is_null() {
                return il2cpp_farproc!(fn(&Il2cppClass) -> *mut T, (*property).get)(self);
            } else {
                null_mut()
            }
        }
    }

    pub fn get_property_value<T: Sized>(&self, property_name: &str) -> Option<&'static mut T> {
        unsafe { self.get_property_value_unsafe::<T>(property_name).as_mut() }
    }

    pub fn set_property_value<T>(&self, property_name: &str, value: T) {
        let c_property_name = CString::new(property_name).unwrap();
        unsafe {
            let func = il2cpp_farproc!(fn(&Il2cppClass, *const i8) -> *mut Il2cppPropertyInfo, FUNCTIONS.m_class_get_property_from_name);
            let property = func(self, c_property_name.as_ptr());
            if !property.is_null() && !(*property).get.is_null() {
                let set_func = il2cpp_farproc!(fn(&Il2cppClass, T), (*property).set);
                set_func(self, value);
            }
        }
    }

    pub fn get_field_value_unsafe<T: Sized>(&self, field: *mut Il2cppFieldInfo) -> *mut T {
        match field.is_null() || unsafe { (*field).offset } < 0 {
            true => null_mut(),
            false => self.get_member_value_unsafe(unsafe { (*field).offset } as isize),
        }
    }

    pub fn get_field_value<T: Sized>(&self, field: *mut Il2cppFieldInfo) -> Option<&'static mut T> {
        unsafe { self.get_field_value_unsafe::<T>(field).as_mut() }
    }
    
    pub fn set_field_value<T: Sized>(&self, field: *mut Il2cppFieldInfo, value: T) {
        if !field.is_null() && unsafe { (*field).offset } >= 0 {
            self.set_member_value(unsafe { (*field).offset } as isize, value);
        }
    }

    pub fn get_member_value_by_name_unsafe<T: Sized>(&self, member_name: &str) -> *mut T {
        let c_member_name = CString::new(member_name).unwrap();
        unsafe {
            let func = il2cpp_farproc!(fn(&Il2cppClass, *const i8) -> *mut Il2cppFieldInfo, FUNCTIONS.m_class_get_field_from_name);
            let field = func(self, c_member_name.as_ptr());

            if !field.is_null() {
                return match (*field).offset >= 0 {
                    true => self.get_member_value_unsafe((*field).offset as isize),
                    false => null_mut(),
                }
            } else {
                return self.get_property_value_unsafe(member_name);
            }
        }
    }

    pub fn get_member_value_by_name<T>(&self, member_name: &str) -> Option<&'static mut T> {
        unsafe { self.get_member_value_by_name_unsafe::<T>(member_name).as_mut() }
    }

    pub fn set_member_value_by_name<T>(&self, member_name: &str, value: T) {
        let c_member_name = CString::new(member_name).unwrap();
        unsafe {
            let func = il2cpp_farproc!(fn(&Il2cppClass, *const i8) -> *mut Il2cppFieldInfo, FUNCTIONS.m_class_get_field_from_name);
            let field = func(self, c_member_name.as_ptr());
            if !field.is_null() {
                if (*field).offset >= 0 {
                    self.set_member_value((*field).offset as isize, value);
                }
            } else {
                self.set_property_value(member_name, value);
            }
        }
    }

    pub fn get_obscured_via_offset_unsafe<T: Default + Copy>(&self, offset: isize) -> *mut T {
        if offset >= 0 {
            unsafe {
                match std::mem::size_of::<T>() {
                    size if size == std::mem::size_of::<f64>() => {
                        let key = *self.get_member_value_unsafe::<usize>(offset);
                        let value = *self.get_member_value_unsafe::<usize>(offset + std::mem::size_of::<usize>() as isize) ^ key;
                        value as *mut T
                    }
                    size if size == std::mem::size_of::<i32>() => {
                        let key = *self.get_member_value_unsafe::<i32>(offset);
                        let value = *self.get_member_value_unsafe::<i32>(offset + std::mem::size_of::<i32>() as isize) ^ key;
                        value as *mut T
                    }
                    size if size == std::mem::size_of::<bool>() => {
                        let key = self.get_member_value_unsafe::<u8>(offset);
                        let value = *self.get_member_value_unsafe::<i32>(offset + std::mem::size_of::<u8>() as isize) ^ *key as i32;
                        value as *mut T
                    }
                    _ => null_mut(),
                }
            }
        } else {
            null_mut()
        }
    }

    pub fn get_obscured_value<T: Default + Copy>(&self, member_name: &str) -> *mut T {
        let c_member_name = CString::new(member_name).unwrap();
        unsafe {
            let func = il2cpp_farproc!(fn(&Il2cppClass, *const i8) -> *mut Il2cppFieldInfo, FUNCTIONS.m_class_get_field_from_name);
            let field = func(self, c_member_name.as_ptr());
            self.get_obscured_via_offset_unsafe(if !field.is_null() { (*field).offset as isize } else { -1 })
        }
    }

    pub fn set_obscured_via_offset<T: Copy>(&self, offset: isize, value: T) {
        if offset >= 0 {
            unsafe {
                match std::mem::size_of::<T>() {
                    size if size == std::mem::size_of::<f64>() => {
                        let key = *self.get_member_value_unsafe::<usize>(offset);
                        let transmuted_value = *std::mem::transmute::<*const T, *const usize>(&value) ^ key;
                        self.set_member_value(offset + std::mem::size_of::<usize>() as isize, transmuted_value);
                    }
                    size if size == std::mem::size_of::<i32>() => {
                        let key = *self.get_member_value_unsafe::<i32>(offset);
                        let transmuted_value = *std::mem::transmute::<*const T, *const i32>(&value) ^ key;
                        self.set_member_value(offset + std::mem::size_of::<i32>() as isize, transmuted_value);
                    }
                    size if size == std::mem::size_of::<bool>() => {
                        let key = *self.get_member_value_unsafe::<u8>(offset);
                        let transmuted_value = *std::mem::transmute::<*const T, *const i32>(&value) ^ key as i32;
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
            let func = il2cpp_farproc!(fn(&Il2cppClass, *const i8) -> *mut Il2cppFieldInfo, FUNCTIONS.m_class_get_field_from_name);
            let field = func(self, c_member_name.as_ptr());
            if !field.is_null() {
                self.set_obscured_via_offset((*field).offset as isize, value);
            }
        }
    }

}

pub enum ClassMemberType {
    Unknown,
    Field,
    Property,
    Method,
}
