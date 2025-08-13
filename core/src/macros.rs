#[macro_export]
macro_rules! create_hook {
    ($hook_name:ident, $class_name:expr, $method_name:expr, $arg_count:expr,
     ($($arg_name:ident : $arg_type:ty),* $(,)?) -> $ret:ty) => {
        static_detour! {
            static $hook_name: unsafe extern "C" fn($($arg_type),*) -> $ret;
        }

        paste::paste! {
            type [<$hook_name _FnRet>] = unsafe extern "C" fn($($arg_type),*) -> $ret;
            type [<$hook_name _FnCb>]  = unsafe extern "C" fn($($arg_type),*);

            static mut [<$hook_name:upper _TARGET>]: ::core::option::Option<[<$hook_name _FnRet>]> = None;
            static mut [<$hook_name:upper _CB>]:     ::core::option::Option<[<$hook_name _FnCb>]>  = None;

            #[no_mangle]
            #[inline(never)]
            fn [<detour_ $hook_name>]($($arg_name : $arg_type),*) -> $ret {
                unsafe {
                    #[cfg(feature = "debug_log_hook_calls")] 
                    ::log::info!("HOOK calling orig, {}", stringify!($hook_name));
                    let r = $hook_name.call($($arg_name),*);
                    #[cfg(feature = "debug_log_hook_calls")] 
                    ::log::info!("HOOK orig call complete, calling callback {}", stringify!($hook_name));
         
                    if let Some(cb) = [<$hook_name:upper _CB>] {
                        cb($($arg_name),*);
                    }

                    #[cfg(feature = "debug_log_hook_calls")] 
                    ::log::info!("HOOK callback call complete, hook {}", stringify!($hook_name));
                                 
                    r
                }
            }

            #[no_mangle]
            #[inline(never)]
            pub fn [<hook_setup_ $hook_name>]() {
                unsafe {
                    let method_ptr =
                        class::get_method_pointer_by_name($class_name, $method_name, $arg_count)
                        .expect("failed to resolve method pointer");

                    let target: [<$hook_name _FnRet>] = ::core::mem::transmute::<*const (), [<$hook_name _FnRet>]>(method_ptr as *const ());
                    [<$hook_name:upper _TARGET>] = Some(target);

                    if let Err(err) = $hook_name.initialize(target, [<detour_ $hook_name>])
                        .and_then(|_| $hook_name.enable())
                    {
                        log::error!(
                            "Failed to set {}::{} detour. adr: {:p}. {:?}",
                            $class_name, $method_name, method_ptr, err
                        );
                    } else {
                        log::info!(
                            "{}::{} detour set. adr: {:p}",
                            $class_name, $method_name, method_ptr
                        );
                    }
                }
            }

            #[no_mangle]
            #[inline(never)]
            pub unsafe extern "C" fn [<set_hook_ $hook_name>](callback: [<$hook_name _FnCb>]) {
                [<$hook_name:upper _CB>] = Some(callback);
                log::info!("{}::{} callback set.", $class_name, $method_name);
            }

            #[no_mangle]
            #[inline(never)]
            pub unsafe extern "C" fn [<clear_callback_ $hook_name>]() {
                [<$hook_name:upper _CB>] = None;
            }

            #[no_mangle]
            #[inline(never)]
            pub unsafe extern "C" fn [<disable_hook_ $hook_name>]() {
                if let Err(err) = $hook_name.disable() {
                    log::error!("Disable {}::{} failed: {:?}", $class_name, $method_name, err);
                }
            }
        }
    };

    ($hook_name:ident, $class_name:expr, $method_name:expr, $arg_count:expr, ($($arg_name:ident : $arg_type:ty),* $(,)?)) => {
        $crate::create_hook!(
            $hook_name, $class_name, $method_name, $arg_count,
            ($($arg_name : $arg_type),*) -> ()
        );
    };
}
