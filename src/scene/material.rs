use glam::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    color: Vec3,
}

impl Material {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }

    pub fn color(&self) -> &Vec3 {
        &self.color
    }
}
