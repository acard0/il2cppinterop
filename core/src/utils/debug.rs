use std::ffi::CStr;
use std::fmt::Write;

use crate::mono::{definitions::object::SystemObject, reflection::{class, meta::Il2cppClass}};

pub fn print_class_members(repr: &Il2cppClass, include_methods: bool) {
    unsafe {
        let s_class = repr.get_class_name();
        let mut str = format!("{}>> ", s_class);

        if include_methods {
            for method in class::get_methods(repr) {
                if let Ok(s_method) = CStr::from_ptr(method.name).to_str() {
                    write!(str, "method: {}, ", s_method).unwrap();
                }
            }
        } else {
            for field in class::get_fields(repr) {
                if let Ok(s_field) = CStr::from_ptr(field.name).to_str() {
                    write!(str, "field: {}, ", s_field).unwrap();
                }
            }
        }

        str.truncate(str.trim_end_matches(", ").len());
        println!(">> {}", str);
    }
}


pub fn intermediate_serialize(object: &SystemObject) {
    unsafe {
        let s_class = object.get_class_name();

        let mut str = format!("");

        for field in class::get_fields(object) {
            if let Ok(s_field) = CStr::from_ptr(field.name).to_str() {
                let value = object.get_member_value_by_field::<i32>(field);
                write!(str, "\"{}\":\"{:?}\", ", s_field, value).unwrap();
            }
        }

        println!("{s_class}(@{object:p}) >> {{{str}}}");
    }
}
