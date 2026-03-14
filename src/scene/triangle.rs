use glam::USizeVec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    vertices: USizeVec3,
    material: usize,
}

impl Triangle {
    pub fn new(vertices: USizeVec3, material: usize) -> Self {
        Self { vertices, material }
    }

    pub fn vertices(&self) -> &USizeVec3 {
        &self.vertices
    }

    pub fn material(&self) -> &usize {
        &self.material
    }
}
