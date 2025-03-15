use crate::IL2CPP_RStr;

IL2CPP_RStr!(IL2CPP_MAIN_MODULE, "GameAssembly.dll");
IL2CPP_RStr!(IL2CPP_INIT, "il2cpp_init");

IL2CPP_RStr!(IL2CPP_CLASS_FROM_NAME, "il2cpp_class_from_name");
IL2CPP_RStr!(IL2CPP_CLASS_GET_NESTED_TYPES, "il2cpp_class_get_nested_types");
IL2CPP_RStr!(IL2CPP_CLASS_GET_FIELDS, "il2cpp_class_get_fields");
IL2CPP_RStr!(IL2CPP_CLASS_GET_FIELD_FROM_NAME, "il2cpp_class_get_field_from_name");
IL2CPP_RStr!(IL2CPP_CLASS_GET_METHODS, "il2cpp_class_get_methods");
IL2CPP_RStr!(IL2CPP_CLASS_GET_METHOD_FROM_NAME, "il2cpp_class_get_method_from_name");
IL2CPP_RStr!(IL2CPP_CLASS_GET_PROPERTY_FROM_NAME, "il2cpp_class_get_property_from_name");
IL2CPP_RStr!(IL2CPP_CLASS_GET_TYPE, "il2cpp_class_get_type");

IL2CPP_RStr!(IL2CPP_TYPE_GET_CLASS, "il2cpp_class_from_type");
IL2CPP_RStr!(IL2CPP_DOMAIN_GET, "il2cpp_domain_get");
IL2CPP_RStr!(IL2CPP_DOMAIN_GET_ASSEMBLIES, "il2cpp_domain_get_assemblies");
IL2CPP_RStr!(IL2CPP_IMAGE_GET_CLASS, "il2cpp_image_get_class");
IL2CPP_RStr!(IL2CPP_IMAGE_GET_CLASS_COUNT, "il2cpp_image_get_class_count");
IL2CPP_RStr!(IL2CPP_RESOLVE_FUNC, "il2cpp_resolve_icall");
IL2CPP_RStr!(IL2CPP_STRING_NEW, "il2cpp_string_new");
IL2CPP_RStr!(IL2CPP_TYPE_GET_OBJECT, "il2cpp_type_get_object");
IL2CPP_RStr!(IL2CPP_OBJECT_NEW, "il2cpp_object_new");
IL2CPP_RStr!(IL2CPP_METHOD_GET_PARAM_NAME, "il2cpp_method_get_param_name");
IL2CPP_RStr!(IL2CPP_METHOD_GET_PARAM, "il2cpp_method_get_param");
IL2CPP_RStr!(IL2CPP_CLASS_FROM_IL2CPP_TYPE, "il2cpp_class_from_il2cpp_type");
IL2CPP_RStr!(IL2CPP_FIELD_STATIC_GET_VALUE, "il2cpp_field_static_get_value");
IL2CPP_RStr!(IL2CPP_FIELD_STATIC_SET_VALUE, "il2cpp_field_static_set_value");
IL2CPP_RStr!(IL2CPP_VALUE_BOX, "il2cpp_value_box");
IL2CPP_RStr!(IL2CPP_OBJECT_UNBOX, "il2cpp_object_unbox");

IL2CPP_RStr!(ILC2PP_ARRAY_NEW, "il2cpp_array_new");

IL2CPP_RStr!(IL2CPP_RUNTIME_INVOKE, "il2cpp_runtime_invoke");

IL2CPP_RStr!(IL2CPP_THREAD_ATTACH, "il2cpp_thread_attach");
IL2CPP_RStr!(IL2CPP_THREAD_DETACH, "il2cpp_thread_detach");
IL2CPP_RStr!(IL2CPP_THREAD_CURRENT, "il2cpp_thread_current");

IL2CPP_RStr!(IL2CPP_ALLOC, "il2cpp_alloc");
IL2CPP_RStr!(IL2CPP_FREE, "il2cpp_free");
IL2CPP_RStr!(IL2CPP_GC_DISABLE, "il2cpp_gc_disable");
IL2CPP_RStr!(IL2CPP_GC_ENABLE, "il2cpp_gc_enable");
IL2CPP_RStr!(IL2CPP_GC_IS_DISABLED, "il2cpp_gc_is_disabled");
IL2CPP_RStr!(IL2CPP_GC_GET_USED_SIZE, "il2cpp_gc_get_used_size");
IL2CPP_RStr!(IL2CPP_GC_GET_HEAP_SIZE, "il2cpp_gc_get_heap_size");
IL2CPP_RStr!(IL2CPP_GC_CREATE_HANDLE, "il2cpp_gchandle_new");
IL2CPP_RStr!(IL2CPP_GC_DESTROY_HANDLE, "il2cpp_gchandle_free");
IL2CPP_RStr!(IL2CPP_GC_WEAKREF_CREATE, "il2cpp_gchandle_new_weakref");
IL2CPP_RStr!(IL2CPP_GC_WEAKREF_GET_TARGET, "il2cpp_gchandle_get_target");
IL2CPP_RStr!(IL2CPP_GC_COLLECT, "il2cpp_gc_collect");
IL2CPP_RStr!(IL2CPP_GC_COLLECT_A_LITTLE, "il2cpp_gc_collect_a_little");

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