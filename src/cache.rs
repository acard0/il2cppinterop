use std::{collections::HashMap, sync::{atomic::AtomicPtr, Mutex}};
use lazy_static::lazy_static;

use crate::il2cpp::builtin::Il2cppObject;

lazy_static! {
    static ref __SYSTEM_TYPES: Mutex<HashMap<u32, AtomicPtr<Il2cppObject>>> = Mutex::new(HashMap::new());
    static ref __INITIALIZERS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub mod system_type_cache {
    use std::{ptr::null_mut, sync::atomic::{AtomicPtr, Ordering}};
    use super::{__SYSTEM_TYPES, __INITIALIZERS, Il2cppObject};
    use crate::il2cpp::{self};

    pub fn add_with_hash(hash: u32, system_type: *mut Il2cppObject) {
        let mut cache = __SYSTEM_TYPES.lock().unwrap();
        cache.insert(hash, AtomicPtr::new(system_type));
    }

    pub fn add_with_name(name: &str, system_type: *mut Il2cppObject) {
        add_with_hash(il2cpp::utils::hash::hash(name), system_type);
    }

    pub fn get_with_hash(hash: u32) -> *mut Il2cppObject {
        let cache = __SYSTEM_TYPES.lock().unwrap();
        cache.get(&hash)
            .map(|ptr| ptr.load(Ordering::SeqCst))
            .unwrap_or(null_mut())
    }

    pub fn get_with_name(name: &str) -> *mut Il2cppObject {
        get_with_hash(il2cpp::utils::hash::hash(name))
    }

    pub mod initializer {
        use il2cpp::api::class;

        use super::*;

        pub fn add_to_be_cached_on_init(name: &str) {
            let mut list = __INITIALIZERS.lock().unwrap();
            list.push(name.to_string());
        }

        pub fn pre_cache() {
            let list: Vec<String>;
            {
                let mut initializer_list = __INITIALIZERS.lock().unwrap();
                list = initializer_list.clone();
                initializer_list.clear();
            }

            for name in list {
                super::add_with_name(&name, class::get_system_type_by_name(&name));
            }
        }
    }
}