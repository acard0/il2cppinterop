use std::ffi::c_void;

use derive_more::derive::Deref;
use meta::Il2cppClass;

use crate::{il2cpp_farproc, mono::reflection::*};

pub trait Il2cppRuntimeMember: PartialEq + std::ops::Deref<Target = &'static mut Il2cppClass> {
    fn equals(&self, other: &Il2cppObject) -> bool {
        unsafe {
            class::get_method_pointer(self, "Equals", 2).is_some_and(|method| {
                il2cpp_farproc!(fn(*mut c_void, *mut c_void) -> bool, method)(
                    self as *const _ as *mut c_void,
                    other as *const _ as *mut c_void,
                )
            })
        }
    }
}

#[derive(Debug, Deref)]
#[repr(C)]
pub struct Il2cppObject {
    #[deref]
    class: &'static mut Il2cppClass,
    monitor: *mut c_void,
}

unsafe impl Send for Il2cppObject {}
unsafe impl Sync for Il2cppObject {}
impl Il2cppRuntimeMember for Il2cppObject {}

impl PartialEq for Il2cppObject {
    /// Compares two objects for equality by using System.Object::Equals icall
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}