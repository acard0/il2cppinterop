use array::Il2cppArray;
use derive_more::derive::Debug;
use il2cppinterop_macros::Mono;

use super::*;

use crate::mono::runtime::*;

#[derive(Debug, Mono)]
#[repr(C)]
pub struct Il2cppList<T: 'static> {
    #[base]
    object: Il2cppObject,
    array: &'static mut Il2cppArray<T>,
    size: i32,
    version: i32,
    sync_root: *mut Il2cppObject
}

impl<T> Il2cppList<T> {
    pub fn as_array(&self) -> &Il2cppArray<T> {
        &*self.array
    }

    pub fn as_array_mut(&mut self) -> &mut Il2cppArray<T> {
        &mut *self.array
    }
}