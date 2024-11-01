

pub fn rot_string(p_string: &str, i_value: i32) -> String {
    let mut s_ret = String::new();
    
    for ch in p_string.chars() {
        let rotated_char = if ch.is_ascii_alphabetic() {
            let ascii_base = if ch.is_ascii_uppercase() { b'A' } else { b'a' } as i32;
            let i_new_value = ch as i32 + i_value;
            let i_max_value = if ch.is_ascii_uppercase() { b'Z' } else { b'z' } as i32;
            
            let mut i_new_value = i_new_value;
            while i_new_value > i_max_value {
                i_new_value = ascii_base + (i_new_value - i_max_value) - 1;
            }
            
            std::char::from_u32(i_new_value as u32).unwrap_or(ch)
        } else {
            ch
        };
        
        s_ret.push(rotated_char);
    }
    
    s_ret
}
