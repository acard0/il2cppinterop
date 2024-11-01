[il2cpp resolver](https://github.com/sneakyevil/IL2CPP_Resolver) but written in rust

# setup
```rust
#[no_mangle]
pub unsafe extern "system" fn DllMain(h_module: HMODULE, fdw_reason: u32, lp_reserved: *mut c_void) -> i32 {
    match fdw_reason {
        1 => { // DLL_PROCESS_ATTACH
            let params = Box::into_raw(Box::new(MainParams { base: h_module.clone(), reserved: lp_reserved })) as usize;
            _= CloseHandle(sys::thread::spawn(move || {            
                let params = Box::from_raw(params as *mut MainParams);

                // resolves il2cpp exports and rest of the common used mono methods using exported reflection call
                il2cppinterop_core::initialize(params.base, Some(Duration::MAX));

                app::main(*params)
            }));
        },
        0 => { // DLL_PROCESS_DEATTACH
           
        }
        _ => {}
    };

    1
}
```

# finding class definition
```rust
    let cz_session = class::find("Game.Session")
        .expect("failed to find Session class");
```

# getting object instances inherited from UnityObject
```rust
// UnityEngine.GameObject::FindObjectsOfType(System.Type,System.Boolean)
    let objects = il2cppinterop_core::unity::object::find_objects_of_type_by_name::<&UnityGameObject>(&UNITY_GAMEOBJECT_CLASS, false)
        .expect("Failed to get UNITY_GAMEOBJECT_CLASS instances");
```

# getting object instances via pattern matching
```rust
  // il2cpp dump
  struct Il2CppObject // low-level object repr, any System.Object starts from here
  {
      Il2CppClass *klass; // first offset is ptr to class meta
      void *monitor;
  };

    let session: Option<*mut Session> = mono::definitions::object::find(cz_session, |address| { unsafe {
        *(address.add(0x8) as *const isize) == 0x0 
        && *(address.add(0xC) as *const isize) == 0x0
        && *(address.add(0x10) as *const isize) == 0x1a
    }}).expect("Encountered error while searching for Game.Session instance");
```

# setting up a hook to intercept calls
```rust
/// Hooks into Game.SessionSystem::Send
unsafe fn hook_setup_SessionSystemSend() {
    // see https://github.com/darfink/detour-rs/forks
    static_detour! { static Hook_SessionSystem_Send: unsafe extern "C" fn(*mut Session, *mut SystemObject); }

    // intercepter
    fn mHook_SessionSystem_Send(_session: *mut Session, _module: *mut SystemObject) { unsafe {
        let module = &*_module;
        debug::intermediate_serialize(module);
       
        Hook_SessionSystem_Send.call(_session, _module);
    }}

    let mp_sessionSystem_Send = class::get_method_pointer_by_name("Game.SessionSystem", "Send", 2)
        .expect("failed to find DarkOrbit.SessionSystem::Send");

    match Hook_SessionSystem_Send
        .initialize(std::mem::transmute(mp_sessionSystem_Send), mHook_SessionSystem_Send)
        .and_then(|_| {
            Hook_SessionSystem_Send.enable()
        }) {
            Ok(_) => log::info!("Game.SessionSystem::Send detour set. Instruction adr: {mp_sessionSystem_Send:p}"),
            Err(err) => log::error!("Failed to set Game.SessionSystem::Send detour. Instruction adr: {mp_sessionSystem_Send:p}. {:?}", err),
        }
}
```

# creating an object reprenstation with 'monomorphism'
```rust
// polymorphism in rust is not built in. to avoid boiler plate code as much as possible
// marking field with #[base] simply sets target for Deref trait. dereferencing all fields and methods from 'parent'.

// still, trait boundaries will require manual implementation.
// currently only PartialEq trait is being chained in favour of Il2cppDictionary<T: PartialEq + ..., V ...>
#[derive(Debug, Mono, Getters, Setters)]
#[getset(set = "pub", get = "pub with_prefix")]
pub struct MoveRequest {
    #[base]
    #[getset(skip)]
    object: SystemObject,
    position_x: i32,
    target_y: i32,
    target_x: i32,
    position_y: i32,
}
```

# creating an object instance
```rust
impl MoveRequest {
    pub fn new() -> &'static mut Self {
        object::new_from_namespace("Game.MoveRequest")
            .expect("Failed to create Game.MoveRequest")
    }
}
```

# finding and calling a method
```rust
il2cpp_farproc! is helper macro for reinterpreting c call on 32bit and fastcall on 64bit
fn send_to_session(session: &mut Session, module: &Il2cppObject) -> bool { unsafe {
    class::get_method_pointer_by_name(&GAME_SESSION_SYSTEM, "Send", 2) // +1 if calling non-static method (this, args...)
        .inspect(|mptr| il2cpp_farproc!(fn(&Session, &Il2cppObject), *mptr) (session, module))
        .is_some()
}}
```
