
pub fn hash(m_string: &str) -> u32 {
    let mut m_hash = 0u32;

    for &byte in m_string.as_bytes() {
        m_hash = m_hash.wrapping_add(byte as u32);
        m_hash = m_hash.wrapping_add(m_hash << 10);
        m_hash ^= m_hash >> 6;
    }

    m_hash = m_hash.wrapping_add(m_hash << 3);
    m_hash ^= m_hash >> 11;
    m_hash = m_hash.wrapping_add(m_hash << 15);

    m_hash
}

pub const fn get_compile_time(m_hashed: &str) -> u32 {
    let mut m_hash = 0u32;
    let bytes = m_hashed.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        m_hash = m_hash.wrapping_add(bytes[i] as u32);
        m_hash = m_hash.wrapping_add(m_hash << 10);
        m_hash ^= m_hash >> 6;
        i += 1;
    }

    m_hash = m_hash.wrapping_add(m_hash << 3);
    m_hash ^= m_hash >> 11;
    m_hash = m_hash.wrapping_add(m_hash << 15);

    m_hash
}

#[macro_export]
macro_rules! il2cpp_hash {
    ($m_string:expr) => {{
        const M_HASH: u32 = il2cpp::utils::hash::get_compile_time($m_string);
        M_HASH
    }};
}