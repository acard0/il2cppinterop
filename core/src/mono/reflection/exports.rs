use crate::IL2CPP_RStr;

IL2CPP_RStr!(IL2CPP_MAIN_MODULE, "GameAssembly.dll");
IL2CPP_RStr!(IL2CPP_INIT_EXPORT, "il2cpp_init");
IL2CPP_RStr!(IL2CPP_CLASS_FROM_NAME_EXPORT, "il2cpp_class_from_name");
IL2CPP_RStr!(IL2CPP_CLASS_GET_FIELDS, "il2cpp_class_get_fields");
IL2CPP_RStr!(IL2CPP_CLASS_GET_FIELD_FROM_NAME_EXPORT, "il2cpp_class_get_field_from_name");
IL2CPP_RStr!(IL2CPP_CLASS_GET_METHODS, "il2cpp_class_get_methods");
IL2CPP_RStr!(IL2CPP_CLASS_GET_METHOD_FROM_NAME_EXPORT, "il2cpp_class_get_method_from_name");
IL2CPP_RStr!(IL2CPP_CLASS_GET_PROPERTY_FROM_NAME_EXPORT, "il2cpp_class_get_property_from_name");
IL2CPP_RStr!(IL2CPP_CLASS_GET_TYPE_EXPORT, "il2cpp_class_get_type");
IL2CPP_RStr!(IL2CPP_TYPE_GET_CLASS_EXPORT, "il2cpp_class_from_type");
IL2CPP_RStr!(IL2CPP_DOMAIN_GET_EXPORT, "il2cpp_domain_get");
IL2CPP_RStr!(IL2CPP_DOMAIN_GET_ASSEMBLIES_EXPORT, "il2cpp_domain_get_assemblies");
IL2CPP_RStr!(IL2CPP_IMAGE_GET_CLASS_EXPORT, "il2cpp_image_get_class");
IL2CPP_RStr!(IL2CPP_IMAGE_GET_CLASS_COUNT_EXPORT, "il2cpp_image_get_class_count");
IL2CPP_RStr!(IL2CPP_RESOLVE_FUNC_EXPORT, "il2cpp_resolve_icall");
IL2CPP_RStr!(IL2CPP_STRING_NEW_EXPORT, "il2cpp_string_new");
IL2CPP_RStr!(IL2CPP_THREAD_ATTACH_EXPORT, "il2cpp_thread_attach");
IL2CPP_RStr!(IL2CPP_THREAD_DETACH_EXPORT, "il2cpp_thread_detach");
IL2CPP_RStr!(IL2CPP_TYPE_GET_OBJECT_EXPORT, "il2cpp_type_get_object");
IL2CPP_RStr!(IL2CPP_OBJECT_NEW, "il2cpp_object_new");
IL2CPP_RStr!(IL2CPP_METHOD_GET_PARAM_NAME, "il2cpp_method_get_param_name");
IL2CPP_RStr!(IL2CPP_METHOD_GET_PARAM, "il2cpp_method_get_param");
IL2CPP_RStr!(IL2CPP_CLASS_FROM_IL2CPP_TYPE, "il2cpp_class_from_il2cpp_type");
IL2CPP_RStr!(IL2CPP_FIELD_STATIC_GET_VALUE, "il2cpp_field_static_get_value");
IL2CPP_RStr!(IL2CPP_FIELD_STATIC_SET_VALUE, "il2cpp_field_static_set_value");

IL2CPP_RStr!(IL2CPP_ALLOC, "il2cpp_alloc");
IL2CPP_RStr!(IL2CPP_FREE, "il2cpp_free");
IL2CPP_RStr!(IL2CPP_GC_DISABLE, "il2cpp_gc_disable");
IL2CPP_RStr!(IL2CPP_GC_ENABLE, "il2cpp_gc_enable");
IL2CPP_RStr!(IL2CPP_GC_GET_USED_SIZE, "il2cpp_gc_get_used_size");
IL2CPP_RStr!(IL2CPP_GC_GET_HEAP_SIZE, "il2cpp_gc_get_heap_size");
IL2CPP_RStr!(IL2CPP_GC_CREATE_HANDLE, "il2cpp_gchandle_new");
IL2CPP_RStr!(IL2CPP_GC_DESTROY_HANDLE, "il2cpp_gchandle_free");

#[cfg(target_pointer_width = "64")]
#[macro_export]
macro_rules! il2cpp_farproc {
    (fn($($arg:ty),*), $func_addr:expr) => {
        {
            #[allow(unused_unsafe)]
            unsafe { std::mem::transmute::<_, unsafe extern "fastcall" fn($($arg),*) -> ()>($func_addr) }
        }
    };
    (fn($($arg:ty),*) -> $ret:ty, $func_addr:expr) => {
        {
            #[allow(unused_unsafe)]
            unsafe { std::mem::transmute::<_, unsafe extern "fastcall" fn($($arg),*) -> $ret>($func_addr) }
        }
    };
    (fn($($arg:ty),* , ...), $func_addr:expr) => {
        {
            #[allow(unused_unsafe)]
            unsafe { std::mem::transmute::<_, unsafe extern "fastcall" fn($($arg),* , ...) -> ()>($func_addr) }
        }
    };
    (fn($($arg:ty),* , ...) -> $ret:ty, $func_addr:expr) => {
        {
            #[allow(unused_unsafe)]
            unsafe { std::mem::transmute::<_, unsafe extern "fastcall" fn($($arg),* , ...) -> $ret>($func_addr) }
        }
    };
}

#[cfg(target_pointer_width = "32")]
#[macro_export]
macro_rules! il2cpp_farproc {
    (fn($($arg:ty),*), $func_addr:expr) => {
        {
            #[allow(unused_unsafe)]
            unsafe { std::mem::transmute::<_, unsafe extern "C" fn($($arg),*) -> ()>($func_addr) }
        }
    };
    (fn($($arg:ty),*) -> $ret:ty, $func_addr:expr) => {
        {
            #[allow(unused_unsafe)]
            unsafe { std::mem::transmute::<_, unsafe extern "C" fn($($arg),*) -> $ret>($func_addr) }
        }
    };
    (fn($($arg:ty),* , ...), $func_addr:expr) => {
        {
            #[allow(unused_unsafe)]
            unsafe { std::mem::transmute::<_, unsafe extern "C" fn($($arg),* , ...) -> ()>($func_addr) }
        }
    };
    (fn($($arg:ty),* , ...) -> $ret:ty, $func_addr:expr) => {
        {
            #[allow(unused_unsafe)]
            unsafe { std::mem::transmute::<_, unsafe extern "C" fn($($arg),* , ...) -> $ret>($func_addr) }
        }
    };
}