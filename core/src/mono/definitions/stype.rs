
use il2cppinterop_macros::Mono;

use super::object::SystemObject;

#[derive(Debug, Mono)]
#[repr(C)]
pub struct SystemType {
    #[base]
    object: SystemObject,
}