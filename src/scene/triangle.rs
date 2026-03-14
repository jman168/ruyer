use glam::USizeVec3;

/// Structure for holding a triangle in "GPU" representation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    vertices: USizeVec3,
    material: usize,
}

impl Triangle {
    /// Creates a new triangle.
    pub fn new(vertices: USizeVec3, material: usize) -> Self {
        Self { vertices, material }
    }

    /// Returns the triangles vertex indices.
    pub fn vertices(&self) -> &USizeVec3 {
        &self.vertices
    }

    /// Returns the triangles material index.
    pub fn material(&self) -> &usize {
        &self.material
    }
}
