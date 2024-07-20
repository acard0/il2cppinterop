use std::{thread, time::Duration};

use crate::{il2cpp::{self, api::domain, builtin::Il2cppArray}, unity::{api::{camera::CCamera, object}, definitions::UNITY_CAMERA_CLASS}, MainParams};

pub unsafe fn main(_params: MainParams) -> u32 {  
    thread::spawn(logical);

    1
}

fn logical() { 
    loop {

        let pth = il2cpp::api::thread::attach(domain::get());
        
        unsafe {
            println!("fething cameras");

            let cameras: *mut Il2cppArray<CCamera> = object::find_objects_of_type_by_name(&UNITY_CAMERA_CLASS, false);
            
            for camera in (*cameras).into_iter() {
                println!("camera instance found at {:p}", camera);
                println!("fov of camera instance {:p} is {}", camera, camera.get_field_of_view());

                camera.set_field_of_view(90.0);
                println!("fov of camera instance {:p} is set to {}", camera, camera.get_field_of_view());
            }
        }

        il2cpp::api::thread::detach(pth);
        thread::sleep(Duration::from_secs(4));
    }
}
