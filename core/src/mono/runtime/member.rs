use std::ffi::c_void;

use derive_more::derive::Deref;
use meta::Il2cppClass;

use crate::{il2cpp_farproc, mono::{definitions::stype::SystemType, reflection::{self, *}}};

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

impl Il2cppObject {
    /// Gets raw address in il2cpp memory
    pub fn as_raw_ptr(&self) -> *const c_void {
        self as *const _ as *const c_void
    }
    
    /// Gets raw address in il2cpp memory
    pub fn as_raw_mut_ptr(&mut self) -> *mut c_void {
        self as *mut _ as *mut c_void
    }

    /// Gets runtime class meta
    pub fn get_runtime_meta(&self) -> &Il2cppClass {
        &self.class
    }

    /// Gets System.Type representing type of this object.
    /// Call made: (exported ::GetType call)
    pub fn type_of(&self) -> &SystemType {
        reflection::class::get_system_type(self.get_runtime_meta())
            .expect("get_system_type on Il2cppObject returned none")
    }
}

impl PartialEq for Il2cppObject {
    /// Compares two objects for equality by using System.Object::Equals icall
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}