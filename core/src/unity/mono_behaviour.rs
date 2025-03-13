use std::ffi::c_void;

use il2cppinterop_macros::Mono;


use super::behaviour::UnityBehaviour;


#[derive(Debug, Mono)]
#[repr(C)]
pub struct UnityMonoBehaviour {
    #[base]
    unity_behaviour: UnityBehaviour,
    pub cancellation_token_source: *mut c_void
}

impl UnityMonoBehaviour {

}