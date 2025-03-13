#[macro_export]
macro_rules! create_hook {
    ($hook_name:ident, $class_name:expr, $method_name:expr, $arg_count:expr, ($($arg_name:ident : $arg_type:ty),*)) => {
        static_detour! {
            static $hook_name: unsafe extern "C" fn($($arg_type),*);
        }

        paste::paste! {
            #[allow(non_snake_case)]
            pub fn [<hook_setup_ $hook_name>]() {
                unsafe {
                    let hook_closure = move |$($arg_name: $arg_type),*| {
                        $hook_name.call($($arg_name),*);
                    };

                    let method_ptr = class::get_method_pointer_by_name($class_name, $method_name, $arg_count)
                        .expect(concat!("failed to find ", $class_name, "::", $method_name));

                    match $hook_name.initialize(std::mem::transmute(method_ptr), hook_closure)
                        .and_then(|_| $hook_name.enable())
                    {
                        Ok(_) => log::info!(
                            "{}::{} detour set. Instruction adr: {:p}",
                            $class_name, $method_name, method_ptr
                        ),
                        Err(err) => log::error!(
                            "Failed to set {}::{} detour. Instruction adr: {:p}. {:?}",
                            $class_name, $method_name, method_ptr, err
                        ),
                    }
                }
            }

            #[no_mangle]
            #[log_call]
            pub unsafe extern "C" fn [<set_hook_ $hook_name>](
                callback: unsafe extern "C" fn($($arg_type),*)
            ) {
                $hook_name.set_detour(move |$($arg_name: $arg_type),*| {
                    $hook_name.call($($arg_name),*);
                    let _ = std::panic::catch_unwind(|| callback($($arg_name),*));
                });

                log::info!(
                    "{}::{} trampoline set -> {:p}",
                    $class_name, $method_name, callback as *const ()
                );
            }
        }
    };
}