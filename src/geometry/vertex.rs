use glam::{Vec2, Vec3};

/// Structure for storing a vertex of a mesh.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
}

impl Vertex {
    /// Creates a new vertex with a given position, normal, and UV coordinate.
    pub fn new(position: Vec3, normal: Vec3, uv: Vec2) -> Self {
        Self {
            position,
            normal,
            uv,
        }
    }

    /// Returns the position of the vertex.
    pub fn position(&self) -> &Vec3 {
        &self.position
    }

    /// Returns the normal of the vertex.
    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    /// Returns the UV coordinate of the vertex.
    pub fn uv(&self) -> &Vec2 {
        &self.uv
    }
}
