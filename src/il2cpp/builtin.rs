#![allow(unused)]

use std::alloc::System;
use std::array::IntoIter;
use std::ffi::c_char;
use std::ffi::c_void;
use std::ffi::CString;
use std::marker::PhantomData;
use std::mem::transmute;
use std::ptr;
use std::mem;

use widestring::U16CString;

use super::interop::Il2cppClass;
use super::FUNCTIONS;

#[derive(Debug)]
#[repr(C)]
pub struct Il2cppObject {
    pub m_p_class: *mut Il2cppClass,
    pub m_p_monitor: *mut c_void,
}
unsafe impl Send for Il2cppObject {}
unsafe impl Sync for Il2cppObject {}

impl Default for Il2cppObject {
    fn default() -> Self {
        unsafe { mem::zeroed()  }
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct Il2cppArrayBounds {
    pub m_u_length: usize,
    pub m_i_lower_bound: i32,
}

#[derive(Debug)]
#[repr(C)]
pub struct Il2cppArray<T> {
    pub il2cpp_object: Il2cppObject,
    pub m_p_bounds: *mut Il2cppArrayBounds,
    pub m_u_max_length: usize,
    pub m_p_first: *mut T,
}

impl<T> Il2cppArray<T> {
    pub fn get_ptr(&self) -> *const *const T {
        &self.m_p_first as *const *mut T as *const *const T
    }

    pub fn get_ptr_mut(&mut self) -> *mut *mut T {
        &mut self.m_p_first as *mut *mut T
    }

    pub fn get_element(&self, index: usize) -> Option<*const T> {
        if index < self.m_u_max_length {
            return Some(unsafe { *self.get_ptr().add(index) });
        }
        
        None
    }

    pub fn get_element_mut(&mut self, index: usize) -> Option<*mut T> {
        if index < self.m_u_max_length {
            return Some(unsafe { *self.get_ptr_mut().add(index) });
        }

        None
    }

    pub fn insert(&mut self, array: &[T], size: usize, index: usize)
    where
        T: Copy,
    {
        let effective_size = if index + size > self.m_u_max_length {
            self.m_u_max_length.saturating_sub(index)
        } else {
            size
        };

        if index < self.m_u_max_length {
            unsafe {
                let ptr = self.get_ptr_mut() as *mut T;
                for (i, &value) in array.iter().take(effective_size).enumerate() {
                    *ptr.add(index + i) = value;
                }
            }
        }
    }

    pub fn fill(&mut self, value: T)
    where
        T: Copy,
    {
        unsafe {
            let ptr = self.get_ptr_mut() as *mut T;
            for i in 0..self.m_u_max_length {
                *ptr.add(i) = value;
            }
        }
    }

    pub fn remove_at(&mut self, index: usize) {
        if index < self.m_u_max_length {
            for i in index..self.m_u_max_length - 1 {
                let next = unsafe { self.get_ptr_mut().add(index + 1) };
                unsafe { *self.get_ptr_mut().add(index) = *next };
            }
            self.m_u_max_length -= 1;
        }
    }

    pub fn remove_range(&mut self, index: usize, count: usize) {
        let count = if count == 0 { 1 } else { count };
        if index + count < self.m_u_max_length {
            for i in index..self.m_u_max_length - count {
                let next = unsafe { self.get_ptr_mut().add(index + count) };
                unsafe { *self.get_ptr_mut().add(index) = *next };
            }
            self.m_u_max_length -= count;
        }
    }

    pub fn remove_all(&mut self) {
        if self.m_u_max_length > 0 {
            unsafe {
                ptr::write_bytes(*self.get_ptr_mut(), 0, self.m_u_max_length);
            }
            self.m_u_max_length = 0;
        }
    }
}


pub struct Il2cppArrayInterator<'a, T> {
    array: &'a Il2cppArray<T>,
    index: usize,
}

impl<'a, T> Iterator for Il2cppArrayInterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.m_u_max_length {
            unsafe {
                self.index += 1;

                if let Some(item) = self.array.get_element(self.index - 1) {
                    return Some(&*item);
                }

                self.next()
            }
        } else {
            None
        }
    }
}

impl<'a, T> IntoIterator for &'a Il2cppArray<T> {
    type Item = &'a T;
    type IntoIter = Il2cppArrayInterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Il2cppArrayInterator {
            array: self,
            index: 0,
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Il2cppList<T> {
    pub il2cpp_object: Il2cppObject,
    pub m_p_list_array: *mut Il2cppArray<T>,
}

impl<T> Il2cppList<T> {
    pub fn to_array(&self) -> &Il2cppArray<T> {
        unsafe { &*self.m_p_list_array }
    }

    pub fn to_array_mut(&mut self) -> &mut Il2cppArray<T> {
        unsafe { &mut *self.m_p_list_array }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Il2cppDictionary<TKey, TValue> {
    pub il2cpp_object: Il2cppObject,
    pub m_p_buckets: *mut Il2cppArray<i32>,
    pub m_p_entries: *mut Il2cppArray<Il2cppDictionaryEntry<TKey, TValue>>,
    pub m_i_count: i32,
    pub m_i_version: i32,
    pub m_i_free_list: i32,
    pub m_i_free_count: i32,
    pub m_p_comparer: *mut c_void,
    pub m_p_keys: *mut c_void,
    pub m_p_values: *mut c_void,
}

impl<TKey, TValue> Default for Il2cppDictionary<TKey, TValue> {
    fn default() -> Self {
        unsafe { mem::zeroed()  }
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct Il2cppDictionaryEntry<TKey, TValue> {
    pub m_i_hash_code: i32,
    pub m_i_next: i32,
    pub m_t_key: TKey,
    pub m_t_value: TValue,
}

impl<TKey, TValue> Il2cppDictionary<TKey, TValue> {
    pub fn get_entry(&self) -> *mut Il2cppDictionaryEntry<TKey, TValue> {
        unsafe { (*self.m_p_entries).get_ptr_mut() as *mut Il2cppDictionaryEntry<TKey, TValue> }
    }

    pub fn get_key_by_index(&self, index: i32) -> Option<TKey>
    where
        TKey: Default + Copy,
    {
        let entry = self.get_entry();
        if !entry.is_null() {
            unsafe { Some((*entry.add(index as usize)).m_t_key) }
        } else {
            None
        }
    }

    pub fn get_value_by_index(&self, index: i32) -> Option<TValue>
    where
        TValue: Default + Copy,
    {
        let entry = self.get_entry();
        if !entry.is_null() {
            unsafe { Some((*entry.add(index as usize)).m_t_value) }
        } else {
            None
        }
    }

    pub fn get_value_by_key(&self, key: TKey) -> Option<TValue>
    where
        TKey: PartialEq + Copy,
        TValue: Default + Copy,
    {
        for i in 0..self.m_i_count {
            let entry = self.get_entry();
            if !entry.is_null() {
                unsafe {
                    if (*entry.add(i as usize)).m_t_key == key {
                        return Some((*entry.add(i as usize)).m_t_value);
                    }
                }
            }
        }
        None
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct SystemString {
    pub il2cpp_object: Il2cppObject,
    pub m_i_length: i32,
    pub m_w_string: [u16; 1024],
}

impl Default for SystemString {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl SystemString {
    pub fn new(m_string: &str) -> *mut SystemString {
        let c_string = CString::new(m_string).unwrap();
        unsafe { transmute::<_, unsafe extern "C" fn(*const c_char) -> *mut SystemString>(FUNCTIONS.m_string_new)(c_string.as_ptr()) }
    }
    
    pub fn clear(&mut self) {
        #![allow(useless_ptr_null_checks)]
        if self as *const _ == ptr::null() {
            return;
        }
        self.m_w_string.iter_mut().for_each(|c| *c = 0);
        self.m_i_length = 0;
    }

    pub fn to_string(&self) -> String {
        #![allow(useless_ptr_null_checks)]
        if self as *const _ == ptr::null() {
            return String::new();
        }
        let slice = &self.m_w_string[..self.m_i_length as usize];
        let u16_cstring = U16CString::from_vec_truncate(slice.to_vec());
        u16_cstring.to_string_lossy()
    }
}