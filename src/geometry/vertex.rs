use glam::{Vec2, Vec3};
#[cfg(test)]
use std::ops::Index;

/// Structure for storing a single vertex.
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

/// A struct used for accessing a vertex by its index in some container.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VertexIdx(pub u32);

#[cfg(test)]
impl Index<VertexIdx> for Vec<Vertex> {
    type Output = Vertex;

    fn index(&self, index: VertexIdx) -> &Self::Output {
        &self[index.0 as usize]
    }
}
