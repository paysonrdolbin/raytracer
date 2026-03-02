use crate::vec3::Vec3;

pub enum Light {
    Ambient(AmbientLight),
    Directional(DirectionalLight),
}

pub struct DirectionalLight {
    pub direction: Vec3,
    pub color: Vec3,
}
impl DirectionalLight {
    pub fn new(direction: Vec3, color: Vec3) -> Self {
        Self { direction, color }
    }
}

pub struct AmbientLight {
    pub color: Vec3,
}
impl AmbientLight {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}