use crate::scene::Material;

use glam::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TriangleRef<'a> {
    vertices: [&'a Vec3; 3],
    material: &'a Material,
}

impl<'a> TriangleRef<'a> {
    pub fn new(vertices: [&'a Vec3; 3], material: &'a Material) -> Self {
        Self { vertices, material }
    }

    pub fn vertices(&self) -> [&'a Vec3; 3] {
        self.vertices
    }

    pub fn material(&self) -> &'a Material {
        self.material
    }
}
