
use getset::Getters;
use il2cppinterop_macros::Mono;

use crate::mono::reflection::meta::Il2cppType;

use super::object::SystemObject;

#[derive(Debug, Mono, Getters)]
#[getset(get = "pub with_prefix")]
#[repr(C)]
pub struct SystemType {
    #[base]
    #[getset(skip)]
    object: SystemObject,
    stype: &'static mut Il2cppType
}