#![allow(dead_code)]

use crate::app::main;
use std::{collections::HashMap, ffi::{c_uchar, c_void, CString}, fs::OpenOptions, thread, time::Duration};

use il2cpp::{api::callback, definitions::*, FUNCTIONS, GLOBALS};
use log::SetLoggerError;
use simplelog::{CombinedLogger, ConfigBuilder, SimpleLogger};
use unity::api::{camera::{self}, component, game_object::{self}, layer_mask, obfuscators, object, rigidbody, transform};
use windows::{core::{s, PCSTR}, Win32::{Foundation::{CloseHandle, FARPROC, GENERIC_READ, HMODULE, INVALID_HANDLE_VALUE}, Storage::FileSystem::{CreateFileA, FILE_CREATION_DISPOSITION, FILE_FLAGS_AND_ATTRIBUTES, FILE_SHARE_MODE}, System::{Console::AllocConsole, LibraryLoader::{DisableThreadLibraryCalls, GetModuleHandleA, GetProcAddress}}}};

mod unity;
mod il2cpp;
mod cache;
mod platform;
mod error;

mod app;

#[cfg(target_os = "windows")]
pub use platform::windows as sys;

pub static mut EXPORT_OBFUSCATION: ExportObfuscationType = ExportObfuscationType::None;
pub static mut ROT_OBFUSCATION_VALUE: i32 = -1;

#[no_mangle]
pub unsafe extern "system" fn DllMain(h_module: HMODULE, fdw_reason: u32, lp_reserved: *mut c_void) -> i32 {
    match fdw_reason {
        1 => { // DLL_PROCESS_ATTACH

            _= DisableThreadLibraryCalls(h_module);
            _= create_console();

            _= logger();

            let params = Box::into_raw(Box::new(MainParams { base: h_module, reserved: lp_reserved })) as usize;
            _= CloseHandle(sys::thread::spawn(move || {            
                let params = Box::from_raw(params as *mut MainParams);
                initialize(params.base, Some(Duration::MAX));

                main(*params)
            }));
        },
        0 => { // DLL_PROCESS_DEATTACH
           
        }
        _ => {}
    };

    1
}

pub unsafe fn initialize(base: HMODULE, timeout: Option<Duration>) -> bool {
    log::info!("Initializing il2cpp"); 

    GLOBALS.m_base = base;
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
            thread::sleep(Duration::from_secs(1));
        }

        log::info!("{} located at {:p}", IL2CPP_MAIN_MODULE.as_str(), GLOBALS.m_assembly.0 as *mut usize);
    }

    let mut init_export_resolved = false;
    for i in 0..ExportObfuscationType::Max as i32 {
        EXPORT_OBFUSCATION = std::mem::transmute(i);

        if let Some(export) = resolve_export(&IL2CPP_INIT_EXPORT) {
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

    log::info!("Initializing Unity"); 

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

    log::info!("Initialization complete");
    true
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

    cache::system_type_cache::initializer::pre_cache();
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

fn initialize_export_map() -> HashMap<String, &'static mut *mut c_void> { unsafe {
    let mut export_map = HashMap::new();

    export_map.insert(IL2CPP_CLASS_FROM_NAME_EXPORT.to_string(), &mut FUNCTIONS.m_class_from_name);
    export_map.insert(IL2CPP_CLASS_GET_FIELDS.to_string(), &mut FUNCTIONS.m_class_get_fields);
    export_map.insert(IL2CPP_CLASS_GET_FIELD_FROM_NAME_EXPORT.to_string(), &mut FUNCTIONS.m_class_get_field_from_name);
    export_map.insert(IL2CPP_CLASS_GET_METHODS.to_string(), &mut FUNCTIONS.m_class_get_methods);
    export_map.insert(IL2CPP_CLASS_GET_METHOD_FROM_NAME_EXPORT.to_string(), &mut FUNCTIONS.m_class_get_method_from_name);
    export_map.insert(IL2CPP_CLASS_GET_PROPERTY_FROM_NAME_EXPORT.to_string(), &mut FUNCTIONS.m_class_get_property_from_name);
    export_map.insert(IL2CPP_CLASS_GET_TYPE_EXPORT.to_string(), &mut FUNCTIONS.m_class_get_type);
    export_map.insert(IL2CPP_DOMAIN_GET_EXPORT.to_string(), &mut FUNCTIONS.m_domain_get);
    export_map.insert(IL2CPP_DOMAIN_GET_ASSEMBLIES_EXPORT.to_string(), &mut FUNCTIONS.m_domain_get_assemblies);
    export_map.insert(IL2CPP_FREE_EXPORT.to_string(), &mut FUNCTIONS.m_free);
    export_map.insert(IL2CPP_IMAGE_GET_CLASS_EXPORT.to_string(), &mut FUNCTIONS.m_image_get_class);
    export_map.insert(IL2CPP_IMAGE_GET_CLASS_COUNT_EXPORT.to_string(), &mut FUNCTIONS.m_image_get_class_count);
    export_map.insert(IL2CPP_RESOLVE_FUNC_EXPORT.to_string(), &mut FUNCTIONS.m_resolve_function);
    export_map.insert(IL2CPP_STRING_NEW_EXPORT.to_string(), &mut FUNCTIONS.m_string_new);
    export_map.insert(IL2CPP_THREAD_ATTACH_EXPORT.to_string(), &mut FUNCTIONS.m_thread_attach);
    export_map.insert(IL2CPP_THREAD_DETACH_EXPORT.to_string(), &mut FUNCTIONS.m_thread_detach);
    export_map.insert(IL2CPP_TYPE_GET_OBJECT_EXPORT.to_string(), &mut FUNCTIONS.m_type_get_object);
    export_map.insert(IL2CPP_OBJECT_NEW.to_string(), &mut FUNCTIONS.m_pobject_new);
    export_map.insert(IL2CPP_METHOD_GET_PARAM_NAME.to_string(), &mut FUNCTIONS.m_method_get_param_name);
    export_map.insert(IL2CPP_METHOD_GET_PARAM.to_string(), &mut FUNCTIONS.m_method_get_param);
    export_map.insert(IL2CPP_CLASS_FROM_IL2CPP_TYPE.to_string(), &mut FUNCTIONS.m_class_from_il2cpp_type);
    export_map.insert(IL2CPP_FIELD_STATIC_GET_VALUE.to_string(), &mut FUNCTIONS.m_field_static_get_value);
    export_map.insert(IL2CPP_FIELD_STATIC_SET_VALUE.to_string(), &mut FUNCTIONS.m_field_static_set_value);

    export_map
}}

#[repr(i32)]
pub enum ExportObfuscationType {
    None = 0,
    Rot = 1,
    Max = 2,
}

pub struct MainParams {
    pub base: HMODULE,
    pub reserved: *mut c_void
}

fn create_console() -> std::io::Result<()> {
    unsafe {
        _= AllocConsole();
        
        let conin = CString::new("CONIN$").unwrap();
        let conout = CString::new("CONOUT$").unwrap();

        let stdin_handle = CreateFileA(PCSTR(conin.as_ptr() as *const c_uchar), GENERIC_READ.0, FILE_SHARE_MODE(0), None, FILE_CREATION_DISPOSITION(3), FILE_FLAGS_AND_ATTRIBUTES(0), None).unwrap();
        let stdout_handle = CreateFileA(PCSTR(conout.as_ptr() as *const c_uchar), GENERIC_READ.0, FILE_SHARE_MODE(0), None, FILE_CREATION_DISPOSITION(3), FILE_FLAGS_AND_ATTRIBUTES(0), None).unwrap();
        let stderr_handle = CreateFileA(PCSTR(conout.as_ptr() as *const c_uchar), GENERIC_READ.0, FILE_SHARE_MODE(0), None, FILE_CREATION_DISPOSITION(3), FILE_FLAGS_AND_ATTRIBUTES(0), None).unwrap();
   
        if stdin_handle == INVALID_HANDLE_VALUE || stdout_handle == INVALID_HANDLE_VALUE || stderr_handle == INVALID_HANDLE_VALUE {
            return Err(std::io::Error::last_os_error());
        }

        // Redirect standard input
        let stdin = OpenOptions::new().read(true).open(conin.as_c_str().to_str().unwrap())?;
        std::mem::forget(stdin); // Prevent closing the handle

        // Redirect standard output
        let stdout = OpenOptions::new().write(true).open(conout.as_c_str().to_str().unwrap())?;
        std::mem::forget(stdout); // Prevent closing the handle

        // Redirect standard error
        let stderr = OpenOptions::new().write(true).open(conout.as_c_str().to_str().unwrap())?;
        std::mem::forget(stderr); // Prevent closing the handle

        Ok(())
    }
}

fn logger() -> Result<(), SetLoggerError> {
    std::env::set_var("RUST_BACKTRACE", "1");
    let config = ConfigBuilder::new().set_location_level(log::LevelFilter::Info).build();
    CombinedLogger::init(
        vec![
            SimpleLogger::new(log::LevelFilter::Info, config),
        ]
    )
}