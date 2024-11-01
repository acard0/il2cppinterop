#![allow(unused)]

use std::f32::consts::PI;

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(f1: f32, f2: f32) -> Self {
        Self { x: f1, y: f2 }
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(f1: f32, f2: f32, f3: f32) -> Self {
        Self { x: f1, y: f2, z: f3 }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn dot(&self, b: &Vector3) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self::new(self.x / len, self.y / len, self.z / len)
        } else {
            *self
        }
    }

    pub fn to_vectors(&self, m_p_forward: &mut Option<Vector3>, m_p_right: &mut Option<Vector3>, m_p_up: &mut Option<Vector3>) {
        let m_f_deg2rad = PI / 180.0;

        let m_f_sin_x = (self.x * m_f_deg2rad).sin();
        let m_f_cos_x = (self.x * m_f_deg2rad).cos();

        let m_f_sin_y = (self.y * m_f_deg2rad).sin();
        let m_f_cos_y = (self.y * m_f_deg2rad).cos();

        let m_f_sin_z = (self.z * m_f_deg2rad).sin();
        let m_f_cos_z = (self.z * m_f_deg2rad).cos();

        if let Some(m_p_forward) = m_p_forward {
            m_p_forward.x = m_f_cos_x * m_f_cos_y;
            m_p_forward.y = -m_f_sin_x;
            m_p_forward.z = m_f_cos_x * m_f_sin_y;
        }

        if let Some(m_p_right) = m_p_right {
            m_p_right.x = -m_f_sin_z * m_f_sin_x * m_f_cos_y + -m_f_cos_z * -m_f_sin_y;
            m_p_right.y = -m_f_sin_z * m_f_cos_x;
            m_p_right.z = -m_f_sin_z * m_f_sin_x * m_f_sin_y + -m_f_cos_z * m_f_cos_y;
        }

        if let Some(m_p_up) = m_p_up {
            m_p_up.x = m_f_cos_z * m_f_sin_x * m_f_cos_y + -m_f_sin_z * -m_f_sin_y;
            m_p_up.y = m_f_cos_z * m_f_cos_x;
            m_p_up.z = m_f_cos_z * m_f_sin_x * m_f_sin_y + -m_f_sin_z * m_f_cos_y;
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn new(f1: f32, f2: f32, f3: f32, f4: f32) -> Self {
        Self { x: f1, y: f2, z: f3, w: f4 }
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub fn new(f1: f32, f2: f32, f3: f32, f4: f32) -> Self {
        Self { x: f1, y: f2, z: f3, w: f4 }
    }

    pub fn euler(&mut self, m_f_x: f32, m_f_y: f32, m_f_z: f32) -> Self {
        let m_f_deg2rad = PI / 180.0;

        let m_f_x = m_f_x * m_f_deg2rad * 0.5;
        let m_f_y = m_f_y * m_f_deg2rad * 0.5;
        let m_f_z = m_f_z * m_f_deg2rad * 0.5;

        let m_f_sin_x = m_f_x.sin();
        let m_f_cos_x = m_f_x.cos();

        let m_f_sin_y = m_f_y.sin();
        let m_f_cos_y = m_f_y.cos();

        let m_f_sin_z = m_f_z.sin();
        let m_f_cos_z = m_f_z.cos();

        self.x = m_f_cos_y * m_f_sin_x * m_f_cos_z + m_f_sin_y * m_f_cos_x * m_f_sin_z;
        self.y = m_f_sin_y * m_f_cos_x * m_f_cos_z - m_f_cos_y * m_f_sin_x * m_f_sin_z;
        self.z = m_f_cos_y * m_f_cos_x * m_f_sin_z - m_f_sin_y * m_f_sin_x * m_f_cos_z;
        self.w = m_f_cos_y * m_f_cos_x * m_f_cos_z + m_f_sin_y * m_f_sin_x * m_f_sin_z;

        *self
    }

    pub fn euler_from_vector(&mut self, m_v_rot: Vector3) -> Self {
        self.euler(m_v_rot.x, m_v_rot.y, m_v_rot.z)
    }

    pub fn to_euler(&self) -> Vector3 {
        let mut m_v_euler = Vector3::default();

        let m_f_dist = (self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w);

        let m_f_test = self.x * self.w - self.y * self.z;
        if m_f_test > 0.4995 * m_f_dist {
            m_v_euler.x = PI * 0.5;
            m_v_euler.y = 2.0 * self.y.atan2(self.x);
            m_v_euler.z = 0.0;
        } else if m_f_test < -0.4995 * m_f_dist {
            m_v_euler.x = PI * -0.5;
            m_v_euler.y = -2.0 * self.y.atan2(self.x);
            m_v_euler.z = 0.0;
        } else {
            m_v_euler.x = (2.0 * (self.w * self.x - self.y * self.z)).asin();
            m_v_euler.y = (2.0 * (self.w * self.y + self.z * self.x)).atan2(1.0 - 2.0 * (self.x * self.x + self.y * self.y));
            m_v_euler.z = (2.0 * (self.w * self.z + self.x * self.y)).atan2(1.0 - 2.0 * (self.z * self.z + self.x * self.x));
        }

        let m_f_rad2deg = 180.0 / PI;
        m_v_euler.x *= m_f_rad2deg;
        m_v_euler.y *= m_f_rad2deg;
        m_v_euler.z *= m_f_rad2deg;

        m_v_euler
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Bounds {
    pub m_v_center: Vector3,
    pub m_v_extents: Vector3,
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Plane {
    pub m_v_normal: Vector3,
    pub f_distance: f32,
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Ray {
    pub m_v_origin: Vector3,
    pub m_v_direction: Vector3,
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Rect {
    pub f_x: f32,
    pub f_y: f32,
    pub f_width: f32,
    pub f_height: f32,
}

impl Rect {
    pub fn new(f1: f32, f2: f32, f3: f32, f4: f32) -> Self {
        Self { f_x: f1, f_y: f2, f_width: f3, f_height: f4 }
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(f_red: f32, f_green: f32, f_blue: f32, f_alpha: f32) -> Self {
        Self { r: f_red, g: f_green, b: f_blue, a: f_alpha }
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Matrix4x4 {
    pub m: [[f32; 4]; 4],
}

impl Matrix4x4 {
    pub fn new() -> Self {
        Self { m: [[0.0; 4]; 4] }
    }

    pub fn index(&self, i: usize) -> &[f32; 4] {
        &self.m[i]
    }

    pub fn index_mut(&mut self, i: usize) -> &mut [f32; 4] {
        &mut self.m[i]
    }
}