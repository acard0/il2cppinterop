#![allow(unused)]

use std::ffi::{c_char, c_void, CStr};
use std::mem;

use derive_more::derive::Deref;

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

#[derive(Debug)]
#[repr(C)]
pub struct Il2cppAssembly {
    pub m_p_image: *mut Il2cppImage,
    pub m_u_token: u32,
    pub m_referenced_assembly_start: i32,
    pub m_referenced_assembly_count: i32,
    pub m_a_name: Il2cppAssemblyName,
}

#[derive(Debug)]
#[repr(C)]
pub struct Il2cppClass {
    type_info: Il2cppClassTypeInformation,
    static_fields: *mut c_void,
    rgctx_data: *mut Il2CppRGCTXData,
    instance_info: Il2CppClassInstanceInfo,
    vtable: [VirtualInvokeData; 255],
}

impl Il2cppClass {
    /// Gets class name
    pub fn get_class_name(&self) -> &str {
        unsafe { CStr::from_ptr(self.type_info.name).to_str().unwrap() }
    }

    /// Gets class namespace
    pub fn get_class_namespace(&self) -> Option<&str> {
        unsafe { self.type_info.namespace.as_ref()
            .and_then(|str| unsafe { CStr::from_ptr(str).to_str().ok() }) }
    }

    /// Gets class path
    pub fn get_class_path(&self) -> String {
        let class = self.get_class_name();
        self.get_class_namespace()
            .map_or(
                class.to_owned(),
                |namespace| format!("{namespace}.{}", class)
            )
    }
 
    /// Gets class type information
    pub fn get_type_info(&self) -> &Il2cppClassTypeInformation {
        &self.type_info
    }

    /// Gets class instance information
    pub fn get_instance_info(&self) -> &Il2CppClassInstanceInfo {
        &self.instance_info
    }

    /// Gets vtable
    pub fn get_vtable(&self) -> &[VirtualInvokeData; 255] {
        &self.vtable
    }

    /// Gets element class if self represents indexable
    pub fn get_element_class(&self) -> Option<&Il2cppClass> {
        unsafe { self.get_type_info().element_class.as_ref() }
    }

    /// Gets size of element represented by this indexable, zero if not an indexable
    pub fn get_element_size(&self) -> usize {
        unsafe { self.get_instance_info().element_size as usize }
    }

    /// Gets parent class
    pub fn get_parent_class(&self) -> Option<&Il2cppClass> {
        unsafe { self.get_type_info().parent.as_ref() }
    }

    /// Checks if this class represents a value type
    pub fn is_value_type(&self) -> bool {
        self.get_parent_class().is_some_and(|class| class.get_class_name() == "ValueType")
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Il2cppClassTypeInformation {
    pub image: *mut Il2cppImage,
    pub gc_desc: *mut c_void,
    pub name: *const c_char,
    pub namespace: *const c_char,
    pub byval_arg: Il2cppType,
    pub this_arg: Il2cppType,
    pub element_class: *mut Il2cppClass,
    pub cast_class: *mut Il2cppClass,
    pub declaring_type: *mut Il2cppClass,
    pub parent: *mut Il2cppClass,
    pub generic_class: *mut c_void,
    pub type_metadata_handle: *mut c_void,
    pub interop_data: *mut c_void,
    pub class: *mut Il2cppClass,
    pub fields: *mut c_void,
    pub events: *mut c_void,
    pub properties: *mut c_void,
    pub methods: *mut c_void,
    pub nested_types: *mut *mut Il2cppClass,
    pub implemented_interfaces: *mut *mut Il2cppClass,
    pub interface_offsets: *mut Il2CppRuntimeInterfaceOffsetPair,
}

#[derive(Debug)]
#[repr(C)]
pub struct Il2CppClassInstanceInfo {
    pub type_hierarchy: *mut *mut Il2cppClass,
    pub unity_user_data: *mut c_void,
    pub initialization_exception_gc_handle: u32,
    pub cctor_started: u32,
    pub cctor_finished: u32,
    pub cctor_thread: usize,
    pub generic_container_handle: *mut c_void,
    pub instance_size: u32,
    #[cfg(feature = "unity_version_2022_3_8f1")] 
    pub unknown: u32,
    pub actual_size: u32,
    pub element_size: u32,
    pub native_size: i32,
    pub static_fields_size: u32,
    pub thread_static_fields_size: u32,
    pub thread_static_fields_offset: i32,
    pub flags: u32,
    pub token: u32,
    pub method_count: u16,
    pub property_count: u16,
    pub field_count: u16,
    pub event_count: u16,
    pub nested_type_count: u16,
    pub vtable_count: u16,
    pub interfaces_count: u16,
    pub interface_offsets_count: u16,
    pub type_hierarchy_depth: u8,
    pub generic_recursion_depth: u8,
    pub rank: u8,
    pub minimum_alignment: u8,
    pub natural_alignment: u8,
    pub packing_size: u8,
    pub bitflags1: u8,
    pub bitflags2: u8,
}

#[derive(Debug)]
#[repr(C)]
pub struct Il2CppRGCTXData {

}

#[derive(Debug)]
#[repr(C)]
pub struct Il2CppRuntimeInterfaceOffsetPair {

}

#[derive(Debug)]
#[repr(C)]
pub struct VirtualInvokeData {
    pub method_ptr: Il2CppMethodPointer,
    pub method_info: *mut Il2cppMethodInfo
}

pub type Il2CppMethodPointer = *mut c_void;

#[cfg(feature = "unity_version_2022_3_8f1")] 
#[derive(Debug)]
#[repr(C)]
pub struct Il2cppType {
    pub data: *mut c_void,
    pub bits: u32,
}

#[cfg(not(feature = "unity_version_2022_3_8f1"))]
#[repr(C)]
pub struct Il2cppType {
    pub data: Il2cppTypeData,
    pub attributes: u16,
    pub utype: u8,
    pub mods: u8,
    pub byref: u8,
    pub pinned: u8,
}

#[cfg(not(feature = "unity_version_2022_3_8f1"))]
#[repr(C)]
pub union Il2cppTypeData {
    pub dummy: *mut c_void,
    pub class_index: u32,
    pub ptype: *mut Il2cppType,
    pub array: *mut c_void,
    pub generic_parameter_index: u32,
    pub generic_class: *mut c_void,
}

#[derive(Debug)]
#[repr(C)]
pub struct Il2cppFieldInfo {
    pub name: *const i8,
    pub field_type: *mut Il2cppType,
    pub parent: *mut Il2cppClass,
    pub offset: i32,
    pub attribute_index: i32,
    pub token: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct Il2cppParameterInfo {
    pub name: *const i8,
    pub position: i32,
    pub token: u32,
    pub parameter_type: *mut Il2cppType,
}

#[repr(C)]
pub struct Il2cppMethodInfo {
    pub method_pointer: *mut c_void,
    #[cfg(feature = "unity_version_2022_3_8f1")]
    pub virtual_method_pointer: *mut c_void,
    pub invoker_method: *mut c_void,
    pub name: *const i8,
    pub class: *mut Il2cppClass,
    pub return_type: *mut Il2cppType,
    #[cfg(feature = "unity_version_2022_3_8f1")]
    pub parameters: *mut *mut Il2cppType,
    #[cfg(not(feature = "unity_version_2022_3_8f1"))]
    pub parameters: *mut Il2cppParameterInfo,
    pub rgctx: Il2cppRGCTXUnion,
    pub generics: Il2cppGenericUnion,
    pub token: u32,
    pub flags: u16,
    pub iflags: u16,
    pub slot: u16,
    pub parameter_count: u8,
    pub bitflags: u8,
}

impl Il2cppMethodInfo {
    /// Gets the method name
    pub fn get_name(&self) -> &str {
        unsafe { CStr::from_ptr(self.name).to_str().unwrap() }
    }
}

#[repr(C)]
pub union Il2cppRGCTXUnion {
    pub rgctx: *mut c_void,
    pub method_definition: *mut c_void,
}

#[repr(C)]
pub union Il2cppGenericUnion {
    pub method: *mut c_void,
    pub container: *mut c_void,
}

#[derive(Debug)]
#[repr(C)]
pub struct Il2cppPropertyInfo {
    pub parent: *mut Il2cppClass,
    pub name: *const i8,
    pub get: *mut Il2cppMethodInfo,
    pub set: *mut Il2cppMethodInfo,
    pub attributes: u32,
    pub token: u32,
}