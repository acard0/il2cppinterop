#![feature(c_variadic)]
#![feature(new_range_api)]

#![allow(static_mut_refs)]
#![allow(dead_code)]

use std::{collections::HashMap, ffi::{c_uchar, c_void, CString}, thread, time::Duration};

use mono::{reflection::{callback, exports::*}, FUNCTIONS, GLOBALS};
use platform::mem;
use unity::*;
use windows::{core::{s, PCSTR}, Win32::{Foundation::{FARPROC, HMODULE}, System::LibraryLoader::{GetModuleHandleA, GetProcAddress}}};

pub mod unity;
pub mod mono;
pub mod cache;
pub mod platform;
pub mod utils;
pub mod error;
pub mod macros;

#[cfg(target_os = "windows")]
pub use platform::windows as sys;

pub static mut EXPORT_OBFUSCATION: ExportObfuscationType = ExportObfuscationType::None;
pub static mut ROT_OBFUSCATION_VALUE: i32 = -1;

pub unsafe fn initialize(timeout: Option<Duration>) -> bool {
    log::info!("Initializing mem"); 
    mem::initialize().unwrap();
    
    log::info!("Initializing il2cpp"); 

    GLOBALS.m_base = GetModuleHandleA(PCSTR::null()).expect("GetModuleHandleA base module returned null");
    log::info!("Main module located at: {:p}", GLOBALS.m_base.0 as *mut usize);
    
    let sz_assembly = PCSTR(CString::new(IL2CPP_MAIN_MODULE.as_str()).unwrap().into_raw() as *mut c_uchar);
    GLOBALS.m_assembly = GetModuleHandleA(sz_assembly).unwrap_or_default();

    if let Some(deadline) = timeout {
        let mut m_seconds_waited = 0;

        while GLOBALS.m_assembly.is_invalid() {
            if m_seconds_waited >= deadline.as_secs() {
                log::error!("Failed to locate {}, timedout.", IL2CPP_MAIN_MODULE.as_str());
                return false;
            }

            GLOBALS.m_assembly = GetModuleHandleA(sz_assembly).unwrap_or_default();
            m_seconds_waited += 1;
            thread::sleep(Duration::from_millis(350));
        }

        log::info!("{} located at {:p}", IL2CPP_MAIN_MODULE.as_str(), GLOBALS.m_assembly.0 as *mut usize);
    }

    let mut init_export_resolved = false;
    for i in 0..ExportObfuscationType::Max as i32 {
        EXPORT_OBFUSCATION = std::mem::transmute(i);

        if let Some(export) = resolve_export(&IL2CPP_INIT) {
            init_export_resolved = true;
            log::info!("Export il2cpp_init located at {:p}", export);
            break;
        }
    }

    if !init_export_resolved {
        log::error!("Failed to resolve export il2cpp_init");
        return false;
    }

    let export_map = initialize_export_map();
    for (name, address) in export_map {
        if let Some(farproc) = resolve_export(&name) {
            *address = farproc as *mut c_void;
            log::info!("Export {} located at {:?}", name, address);
            continue;
        }

        log::error!("Failed to located export {}", name);
        return false;
    }

    log::info!("il2cpp::init. OK"); 

    let unity_player = GetModuleHandleA(s!("UnityPlayer.dll"));
    if unity_player.is_err() {
        log::error!("Failed to locate UnityPlayer module. {:?}", unity_player.err().unwrap());
        return false;
    } else {
        GLOBALS.m_unity_player = unity_player.unwrap();
        log::info!("UnityPlayer.dll located at {:p}", GLOBALS.m_unity_player.0 as *mut usize);
    } 

    if !initialize_unity() {
        return false;
    }

    log::info!("unity::init. OK"); 
    true
}

unsafe fn resolve_export(name: &str) -> FARPROC {
    let cname = CString::new(name).unwrap();
    match EXPORT_OBFUSCATION {
        ExportObfuscationType::Rot => {
            if ROT_OBFUSCATION_VALUE == -1 {
                for i in 1..26 {
                    let obfuscated_name = obfuscators::rot_string(name, i);
                    let c_obfuscated_name = CString::new(obfuscated_name).unwrap();
                    let result = GetProcAddress(GLOBALS.m_assembly, PCSTR(c_obfuscated_name.as_ptr() as *const c_uchar));

                    if result.is_none() {
                        ROT_OBFUSCATION_VALUE = i;
                        return result;
                    }
                }
                
                return None;
            }

            let obfuscated_name = obfuscators::rot_string(name, ROT_OBFUSCATION_VALUE);
            let c_obfuscated_name = CString::new(obfuscated_name).unwrap();
            GetProcAddress(GLOBALS.m_assembly, PCSTR(c_obfuscated_name.as_ptr() as *const c_uchar))
        }
        _ => GetProcAddress(GLOBALS.m_assembly, PCSTR(cname.as_ptr() as *const c_uchar)),
    }
}

unsafe fn initialize_unity() -> bool {
    camera::initialize();
    component::initialize();
    game_object::initialize();
    layer_mask::initialize();
    object::initialize();
    rigidbody::initialize();
    transform::initialize();
    callback::initialize();
    time::initialize();

    cache::system_type_cache::initializer::pre_cache();
    true
}

fn initialize_export_map() -> HashMap<String, &'static mut *mut c_void> { unsafe {
    let mut export_map = HashMap::new();

    export_map.insert(IL2CPP_CLASS_FROM_NAME.to_string(), &mut FUNCTIONS.m_class_from_name);
    export_map.insert(IL2CPP_CLASS_GET_NESTED_TYPES.to_string(), &mut FUNCTIONS.m_class_get_nested_classes);
    export_map.insert(IL2CPP_CLASS_GET_FIELDS.to_string(), &mut FUNCTIONS.m_class_get_fields);
    export_map.insert(IL2CPP_CLASS_GET_FIELD_FROM_NAME.to_string(), &mut FUNCTIONS.m_class_get_field_from_name);
    export_map.insert(IL2CPP_CLASS_GET_METHODS.to_string(), &mut FUNCTIONS.m_class_get_methods);
    export_map.insert(IL2CPP_CLASS_GET_METHOD_FROM_NAME.to_string(), &mut FUNCTIONS.m_class_get_method_from_name);
    export_map.insert(IL2CPP_CLASS_GET_PROPERTY_FROM_NAME.to_string(), &mut FUNCTIONS.m_class_get_property_from_name);
    export_map.insert(IL2CPP_CLASS_GET_TYPE.to_string(), &mut FUNCTIONS.m_class_get_type);
    export_map.insert(IL2CPP_CLASS_FROM_IL2CPP_TYPE.to_string(), &mut FUNCTIONS.m_class_from_il2cpp_type);

    export_map.insert(IL2CPP_TYPE_GET_CLASS.to_string(), &mut FUNCTIONS.m_type_get_class);
    export_map.insert(IL2CPP_DOMAIN_GET.to_string(), &mut FUNCTIONS.m_domain_get);
    export_map.insert(IL2CPP_DOMAIN_GET_ASSEMBLIES.to_string(), &mut FUNCTIONS.m_domain_get_assemblies);
    export_map.insert(IL2CPP_IMAGE_GET_CLASS.to_string(), &mut FUNCTIONS.m_image_get_class);
    export_map.insert(IL2CPP_IMAGE_GET_CLASS_COUNT.to_string(), &mut FUNCTIONS.m_image_get_class_count);
    export_map.insert(IL2CPP_RESOLVE_FUNC.to_string(), &mut FUNCTIONS.m_resolve_function);
    export_map.insert(IL2CPP_STRING_NEW.to_string(), &mut FUNCTIONS.m_string_new);
    export_map.insert(IL2CPP_OBJECT_NEW.to_string(), &mut FUNCTIONS.m_object_new);
    export_map.insert(IL2CPP_TYPE_GET_OBJECT.to_string(), &mut FUNCTIONS.m_type_get_object);
    export_map.insert(IL2CPP_METHOD_GET_PARAM_NAME.to_string(), &mut FUNCTIONS.m_method_get_param_name);
    export_map.insert(IL2CPP_METHOD_GET_PARAM.to_string(), &mut FUNCTIONS.m_method_get_param);
    export_map.insert(IL2CPP_FIELD_STATIC_GET_VALUE.to_string(), &mut FUNCTIONS.m_field_static_get_value);
    export_map.insert(IL2CPP_FIELD_STATIC_SET_VALUE.to_string(), &mut FUNCTIONS.m_field_static_set_value);
    export_map.insert(IL2CPP_OBJECT_UNBOX.to_string(), &mut FUNCTIONS.m_object_unbox);
    export_map.insert(IL2CPP_VALUE_BOX.to_string(), &mut FUNCTIONS.m_value_box);

    export_map.insert(ILC2PP_ARRAY_NEW.to_string(), &mut FUNCTIONS.m_array_new);
    
    export_map.insert(IL2CPP_RUNTIME_INVOKE.to_string(), &mut FUNCTIONS.m_runtime_invoke);
    
    export_map.insert(IL2CPP_THREAD_ATTACH.to_string(), &mut FUNCTIONS.m_thread_attach);
    export_map.insert(IL2CPP_THREAD_DETACH.to_string(), &mut FUNCTIONS.m_thread_detach);
    export_map.insert(IL2CPP_THREAD_CURRENT.to_string(), &mut FUNCTIONS.m_thread_current);

    export_map.insert(IL2CPP_ALLOC.to_string(), &mut FUNCTIONS.m_alloc);
    export_map.insert(IL2CPP_FREE.to_string(), &mut FUNCTIONS.m_free);
    export_map.insert(IL2CPP_GC_DISABLE.to_string(), &mut FUNCTIONS.m_gc_disable);
    export_map.insert(IL2CPP_GC_ENABLE.to_string(), &mut FUNCTIONS.m_gc_enable);
    export_map.insert(IL2CPP_GC_IS_DISABLED.to_string(), &mut FUNCTIONS.m_gc_is_disabled);
    export_map.insert(IL2CPP_GC_GET_USED_SIZE.to_string(), &mut FUNCTIONS.m_gc_get_used_size);
    export_map.insert(IL2CPP_GC_GET_HEAP_SIZE.to_string(), &mut FUNCTIONS.m_gc_get_heap_size);
    export_map.insert(IL2CPP_GC_CREATE_HANDLE.to_string(), &mut FUNCTIONS.m_gc_create_handle);
    export_map.insert(IL2CPP_GC_DESTROY_HANDLE.to_string(), &mut FUNCTIONS.m_gc_destroy_handle);
    export_map.insert(IL2CPP_GC_WEAKREF_CREATE.to_string(), &mut FUNCTIONS.m_gc_create_weakref);
    export_map.insert(IL2CPP_GC_WEAKREF_GET_TARGET.to_string(), &mut FUNCTIONS.m_gc_weakref_get_target);
    export_map.insert(IL2CPP_GC_COLLECT.to_string(), &mut FUNCTIONS.m_gc_collect);
    export_map.insert(IL2CPP_GC_COLLECT_A_LITTLE.to_string(), &mut FUNCTIONS.m_gc_collect_a_little);

    export_map
}}

pub struct MainParams {
    pub base: HMODULE,
    pub reserved: *mut c_void
}

#[repr(i32)]
pub enum ExportObfuscationType {
    None = 0,
    Rot = 1,
    Max = 2,
}