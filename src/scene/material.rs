use glam::Vec3;

/// Structure for holding material properties of an object.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    color: Vec3,
    emission: Vec3,
}

impl Material {
    /// Creates a new material.
    pub fn new(color: Vec3, emission: Vec3) -> Self {
        Self { color, emission }
    }

    /// Returns the color of the material.
    pub fn color(&self) -> &Vec3 {
        &self.color
    }

    pub fn emission(&self) -> &Vec3 {
        &self.emission
    }
}
