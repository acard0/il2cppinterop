use std::{collections::HashMap, sync::{atomic::AtomicPtr, Mutex}};
use lazy_static::lazy_static;

use crate::mono::definitions::object::SystemObject;

lazy_static! {
    static ref __SYSTEM_TYPES: Mutex<HashMap<u32, AtomicPtr<SystemObject>>> = Mutex::new(HashMap::new());
    static ref __INITIALIZERS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub mod system_type_cache {
    use std::{ptr::null_mut, sync::atomic::{AtomicPtr, Ordering}};
    use super::{__SYSTEM_TYPES, __INITIALIZERS, SystemObject};
    use crate::{mono::{self}, utils};

    pub fn add_with_hash(hash: u32, system_type: *mut SystemObject) {
        let mut cache = __SYSTEM_TYPES.lock().unwrap();
        cache.insert(hash, AtomicPtr::new(system_type));
    }

    pub fn add_with_name(name: &str, system_type: *mut SystemObject) {
        add_with_hash(utils::hash::hash(name), system_type);
    }

    pub fn get_with_hash(hash: u32) -> *mut SystemObject {
        let cache = __SYSTEM_TYPES.lock().unwrap();
        cache.get(&hash)
            .map(|ptr| ptr.load(Ordering::SeqCst))
            .unwrap_or(null_mut())
    }

    pub fn get_with_name(name: &str) -> *mut SystemObject {
        get_with_hash(utils::hash::hash(name))
    }

    pub mod initializer {
        use mono::reflection::class;

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
                let next = class::get_system_type_by_name(&name)
                    .expect(&format!("type {} not found to be added into typeof cache", name));
                super::add_with_name(&name, next as *const _ as *mut SystemObject);
            }
        }
    }
}