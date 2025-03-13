use il2cppinterop_macros::Mono;


use super::component::UnityComponent;


#[derive(Debug, Mono)]
#[repr(C)]
pub struct UnityBehaviour {
    #[base]
    unity_object: UnityComponent
}

impl UnityBehaviour {

}