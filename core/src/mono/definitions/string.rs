use std::{ffi::{c_char, CString}, mem::transmute, ptr};

use derive_more::derive::Debug;
use getset::Getters;
use il2cppinterop_macros::Mono;
use widestring::U16CString;

use crate::mono::FUNCTIONS;

use super::object::SystemObject;

pub type SystemStringXRef = &'static mut SystemString;

#[derive(Debug, Mono, Getters)]
#[repr(C)]
pub struct SystemString {
    #[base]
    object: SystemObject,
    #[getset(get = "pub with_prefix")]
    lenght: i32,
    #[debug("{}", self.to_string())]
    wide_string: [u16; 1024 * 1024],
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
        self.wide_string.iter_mut().for_each(|c| *c = 0);
        self.lenght = 0;
    }

    pub fn to_string(&self) -> String {
        #![allow(useless_ptr_null_checks)]
        if self as *const _ == ptr::null() || self.lenght <= 0 || self.lenght > 1024 * 1024 {
            return String::new();
        }
        let slice = &self.wide_string[..self.lenght as usize];
        let u16_cstring = U16CString::from_vec_truncate(slice.to_vec());
        u16_cstring.to_string_lossy()
    }
}

impl std::fmt::Display for SystemString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}