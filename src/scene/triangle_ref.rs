use crate::scene::Material;

use glam::Vec3;

/// Referance to an expanded triangle structure.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TriangleRef<'a> {
    vertices: [&'a Vec3; 3],
    material: &'a Material,
}

impl<'a> TriangleRef<'a> {
    /// Creates a new triangle referance.
    pub fn new(vertices: [&'a Vec3; 3], material: &'a Material) -> Self {
        Self { vertices, material }
    }

    /// Returns the vertices of the triangle.
    pub fn vertices(&self) -> [&'a Vec3; 3] {
        self.vertices
    }

    /// Returns the material of the triangle.
    pub fn material(&self) -> &'a Material {
        self.material
    }
}
