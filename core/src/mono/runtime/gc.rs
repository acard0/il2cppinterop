use crate::{il2cpp_farproc, *};


pub fn suspend() {
    let func = il2cpp_farproc!(fn(), &IL2CPP_GC_DISABLE);
    unsafe { func(); }
}

pub fn resume() {
    let func = il2cpp_farproc!(fn(), &IL2CPP_GC_ENABLE);
    unsafe { func(); }
}