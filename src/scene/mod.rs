//! Structures related to storing and manipulating scenes to be rendered.

use glam::Vec3;

mod triangle_ref;
pub use triangle_ref::TriangleRef;

mod triangle_idx;
pub use triangle_idx::TriangleIdx;

mod material;
pub use material::Material;

/// Hold an entire scene which can then be manipulated or rendered.
#[derive(Debug, Clone, PartialEq)]
pub struct Scene {
    vertices: Vec<Vec3>,
    materials: Vec<Material>,
    triangles: Vec<TriangleIdx>,
}

impl Scene {
    /// Creates a new scene given a set of vertices, materials, and triangles.
    pub fn new(
        vertices: impl Into<Vec<Vec3>>,
        materials: impl Into<Vec<Material>>,
        triangles: impl Into<Vec<TriangleIdx>>,
    ) -> Self {
        Self {
            vertices: vertices.into(),
            materials: materials.into(),
            triangles: triangles.into(),
        }
    }

    /// Returns an iterator which yields references to all the triangles in the scene.
    pub fn triangles<'a>(&'a self) -> impl Iterator<Item = TriangleRef<'a>> {
        self.triangles.iter().map(|t| {
            let vertices = t.vertices();

            TriangleRef::new(
                [
                    &self.vertices[vertices[0]],
                    &self.vertices[vertices[1]],
                    &self.vertices[vertices[2]],
                ],
                &self.materials[*t.material()],
            )
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::geometry::Triangle;
    use glam::{usizevec3, vec3};

    #[test]
    fn test_triangles() {
        let scene = Scene::new(
            [
                vec3(1.0, 1.0, 0.0),
                vec3(1.0, 0.0, 0.0),
                vec3(0.0, 0.0, 0.0),
                vec3(0.0, 1.0, 0.0),
            ],
            [
                Material::new(vec3(1.0, 0.0, 0.0)),
                Material::new(vec3(0.0, 0.0, 1.0)),
            ],
            [
                TriangleIdx::new(usizevec3(0, 1, 2), 0),
                TriangleIdx::new(usizevec3(2, 3, 0), 1),
            ],
        );

        let mut triangles = scene.triangles();

        let triangle = triangles.next().unwrap();
        assert_eq!(triangle.vertices()[0], &vec3(1.0, 1.0, 0.0));
        assert_eq!(triangle.vertices()[1], &vec3(1.0, 0.0, 0.0));
        assert_eq!(triangle.vertices()[2], &vec3(0.0, 0.0, 0.0));
        assert_eq!(triangle.material(), &Material::new(vec3(1.0, 0.0, 0.0)));

        let triangle = triangles.next().unwrap();
        assert_eq!(triangle.vertices()[0], &vec3(0.0, 0.0, 0.0));
        assert_eq!(triangle.vertices()[1], &vec3(0.0, 1.0, 0.0));
        assert_eq!(triangle.vertices()[2], &vec3(1.0, 1.0, 0.0));
        assert_eq!(triangle.material(), &Material::new(vec3(0.0, 0.0, 1.0)));

        assert_eq!(triangles.next(), None);
    }
}
