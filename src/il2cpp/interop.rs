#![allow(unused)]

use std::ffi::c_void;
use std::mem;

#[derive(Debug, PartialEq, Eq)]
pub enum TypeEnum {
    TypeVoid = 1,
    TypeBoolean = 2,
    TypeCharacter = 3,
    TypeInteger = 8,
    TypeFloat = 12,
    TypeString = 14,
    TypePointer = 15,
    TypeValueType = 17,
    TypeClass = 18,
    TypeVariable = 19,
    TypeArray = 20,
    TypeEnum = 85,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FieldAttribute {
    FieldAttributeCompiler,
    FieldAttributePrivate,
    FieldAttributeFamilyAndAssembly,
    FieldAttributeAssembly,
    FieldAttributeFamily,
    FieldAttributeFamilyOrAssembly,
    FieldAttributePublic,
    FieldAttributeAccessMask,
    FieldAttributeStatic = 16,
    FieldAttributeInitOnly = 32,
    FieldAttributeLiteral = 64,
    FieldAttributeNotSerialized = 128,
    FieldAttributeHasRVA = 256,
    FieldAttributeSpecialName = 512,
    FieldAttributeRTSpecialName = 1024,
    FieldAttributeHasMarshal = 4096,
    FieldAttributeInvokeImpl = 8192,
    FieldAttributeDefault = 32768,
    FieldAttributeReserved = 38144,
}

#[derive(Debug)]
#[repr(C)]
pub struct Il2cppImage {
    pub m_p_name: *const i8,
    pub m_p_name_no_ext: *const i8,
}

impl Default for Il2cppImage {
    fn default() -> Self {
        unsafe { mem::zeroed()  }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Il2cppAssemblyName {
    pub m_p_name: *const i8,
    pub m_p_culture: *const i8,
    pub m_p_hash: *const i8,
    pub m_p_public_key: *const i8,
    pub m_u_hash: u32,
    pub m_i_hash_length: i32,
    pub m_u_flags: u32,
    pub m_i_major: i32,
    pub m_i_minor: i32,
    pub m_i_build: i32,
    pub m_b_revision: i32,
    pub m_u_public_key_token: [u8; 8],
}

impl Default for Il2cppAssemblyName {
    fn default() -> Self {
        unsafe { mem::zeroed()  }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Il2cppAssembly {
    pub m_p_image: *mut Il2cppImage,
    pub m_u_token: u32,
    pub m_referenced_assembly_start: i32,
    pub m_referenced_assembly_count: i32,
    pub m_a_name: Il2cppAssemblyName,
}

impl Default for Il2cppAssembly {
    fn default() -> Self {
        unsafe { mem::zeroed()  }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Il2cppClass {
    pub m_p_image: *mut c_void,
    pub m_p_gc: *mut c_void,
    pub m_p_name: *const i8,
    pub m_p_namespace: *const i8,
    pub m_p_value: *mut c_void,
    pub m_p_args: *mut c_void,
    pub m_p_element_class: *mut Il2cppClass,
    pub m_p_cast_class: *mut Il2cppClass,
    pub m_p_declare_class: *mut Il2cppClass,
    pub m_p_parent_class: *mut Il2cppClass,
    pub m_p_generic_class: *mut c_void,
    pub m_p_type_definition: *mut c_void,
    pub m_p_interop_data: *mut c_void,
    pub m_p_fields: *mut c_void,
    pub m_p_events: *mut c_void,
    pub m_p_properties: *mut c_void,
    pub m_p_methods: *mut *mut c_void,
    pub m_p_nested_types: *mut *mut Il2cppClass,
    pub m_implemented_interfaces: *mut *mut Il2cppClass,
    pub m_p_interface_offsets: *mut c_void,
    pub m_p_static_fields: *mut c_void,
    pub m_p_rgctx: *mut c_void,
}

impl Default for Il2cppClass {
    fn default() -> Self {
        unsafe { mem::zeroed()  }
    }
}

#[cfg(feature = "unity_version_2022_3_8f1")]
#[derive(Debug, Default)]
#[repr(C)]
pub struct Il2cppType {
    pub data: *mut c_void,
    pub bits: u32,
}

#[cfg(feature = "unity_version_2022_3_8f1")]
impl Default for Il2cppType {
    fn default() -> Self {
        unsafe { mem::zeroed()  }
    }
}


#[cfg(not(feature = "unity_version_2022_3_8f1"))]
#[repr(C)]
pub struct Il2cppType {
    pub data: Il2cppTypeData,
    pub m_u_attributes: u16,
    pub m_u_type: u8,
    pub m_u_mods: u8,
    pub m_u_byref: u8,
    pub m_u_pinned: u8,
}

#[cfg(not(feature = "unity_version_2022_3_8f1"))]
impl Default for Il2cppType {
    fn default() -> Self {
        unsafe { mem::zeroed()  }
    }
}

#[repr(C)]
pub union Il2cppTypeData {
    pub m_p_dummy: *mut c_void,
    pub m_u_class_index: u32,
    pub m_p_type: *mut Il2cppType,
    pub m_p_array: *mut c_void,
    pub m_u_generic_parameter_index: u32,
    pub m_p_generic_class: *mut c_void,
}

impl Default for Il2cppTypeData {
    fn default() -> Self {
        unsafe { mem::zeroed()  }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Il2cppFieldInfo {
    pub m_p_name: *const i8,
    pub m_p_type: *mut Il2cppType,
    pub m_p_parent_class: *mut Il2cppClass,
    pub m_i_offset: i32,
    pub m_i_attribute_index: i32,
    pub m_u_token: u32,
}

impl Default for Il2cppFieldInfo {
    fn default() -> Self {
        unsafe { mem::zeroed()  }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Il2cppParameterInfo {
    pub m_p_name: *const i8,
    pub m_i_position: i32,
    pub m_u_token: u32,
    pub m_p_parameter_type: *mut Il2cppType,
}

impl Default for Il2cppParameterInfo {
    fn default() -> Self {
        unsafe { mem::zeroed()  }
    }
}

#[repr(C)]
pub struct Il2cppMethodInfo {
    pub m_p_method_pointer: *mut c_void,
    #[cfg(feature = "unity_version_2022_3_8f1")]
    pub m_p_virtual_method_pointer: *mut c_void,
    pub m_p_invoker_method: *mut c_void,
    pub m_p_name: *const i8,
    pub m_p_class: *mut Il2cppClass,
    pub m_p_return_type: *mut Il2cppType,
    #[cfg(feature = "unity_version_2022_3_8f1")]
    pub m_p_parameters: *mut *mut Il2cppType,
    #[cfg(not(feature = "unity_version_2022_3_8f1"))]
    pub m_p_parameters: *mut Il2cppParameterInfo,
    pub m_p_rgctx: Il2cppRGCTXUnion,
    pub m_p_generic: Il2cppGenericUnion,
    pub m_u_token: u32,
    pub m_u_flags: u16,
    pub m_u_flags2: u16,
    pub m_u_slot: u16,
    pub m_u_args_count: u8,
    pub m_u_generic: u8,
    pub m_u_inflated: u8,
    pub m_u_wrapper_type: u8,
    pub m_u_marshaled_from_native: u8,
}

impl Default for Il2cppMethodInfo {
    fn default() -> Self {
        unsafe { mem::zeroed()  }
    }
}

#[repr(C)]
pub union Il2cppRGCTXUnion {
    pub m_p_rgctx: *mut c_void,
    pub m_p_method_definition: *mut c_void,
}

impl Default for Il2cppRGCTXUnion {
    fn default() -> Self {
        unsafe { mem::zeroed()  }
    }
}

#[repr(C)]
pub union Il2cppGenericUnion {
    pub m_p_generic_method: *mut c_void,
    pub m_p_generic_container: *mut c_void,
}

impl Default for Il2cppGenericUnion {
    fn default() -> Self {
        unsafe { mem::zeroed()  }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Il2cppPropertyInfo {
    pub m_p_parent_class: *mut Il2cppClass,
    pub m_p_name: *const i8,
    pub m_p_get: *mut Il2cppMethodInfo,
    pub m_p_set: *mut Il2cppMethodInfo,
    pub m_u_attributes: u32,
    pub m_u_token: u32,
}

impl Default for Il2cppPropertyInfo {
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
