use std::ptr::null_mut;

use crate::{il2cpp::builtin::Il2cppArray, unity::{api::{component::CComponent, game_object::CGameObject, object::find_objects_of_type_by_name}, definitions::{UNITY_GAMEOBJECT_CLASS, UNITY_MONOBEHAVIOUR_CLASS}}};


pub fn get_mono_behaviour() -> *mut CComponent {
    let objects: *mut Il2cppArray<CGameObject> = find_objects_of_type_by_name::<CGameObject>(&UNITY_GAMEOBJECT_CLASS, false);

    if objects.is_null() {
        return null_mut();
    }

    unsafe {
        for object in (*objects).into_iter() {
            if let Some(bhv) = (*object).get_component_of_stype_at_index(&UNITY_MONOBEHAVIOUR_CLASS, 0) {
                return bhv;
            }
        }
    }

    null_mut()
}