//! Structures related to storing and manipulating scenes to be rendered.

use crate::geometry::Vertex;

mod triangle_ref;
pub use triangle_ref::TriangleRef;

mod triangle_idx;
pub use triangle_idx::TriangleIdx;

mod material;
pub use material::Material;

mod gltf;

use crate::geometry::{Ray, RayIntersection, Triangle};

/// Hold an entire scene which can then be manipulated or rendered.
#[derive(Debug, Clone, PartialEq)]
pub struct Scene {
    vertices: Vec<Vertex>,
    materials: Vec<Material>,
    triangles: Vec<TriangleIdx>,
}

impl Scene {
    /// Creates a new scene given a set of vertices, materials, and triangles.
    pub fn new(
        vertices: impl Into<Vec<Vertex>>,
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

    /// Returns [`RayIntersection`] for the closest intersection in the scene should it exist.
    pub fn ray_intersection<'a, 'b>(
        &'a self,
        ray: &'b Ray,
    ) -> Option<(RayIntersection<'b>, TriangleRef<'a>)> {
        let mut best_intersection: Option<(RayIntersection<'b>, TriangleRef<'a>)> = None;

        for triangle in self.triangles() {
            // If the ray intersects the triangle.
            if let Some(intersection) = triangle.ray_intersection(ray) {
                // If we have a best intersection.
                if let Some((best_intersection, best_triangle)) = &mut best_intersection {
                    // If this intersection is closer to the origin, we keep it.
                    if intersection.t() < best_intersection.t() {
                        *best_intersection = intersection;
                        *best_triangle = triangle;
                    }
                }
                // If we don't have a best intersection yet, this one defaults to the best.
                else {
                    best_intersection = Some((intersection, triangle));
                }
            }
        }

        best_intersection
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::geometry::Triangle;
    use glam::{usizevec3, vec2, vec3};

    #[test]
    fn test_triangles() {
        let scene = Scene::new(
            [
                Vertex::new(vec3(1.0, 1.0, 0.0), vec3(0.0, 0.0, 1.0), vec2(0.0, 0.0)),
                Vertex::new(vec3(1.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0), vec2(0.0, 0.0)),
                Vertex::new(vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0), vec2(0.0, 0.0)),
                Vertex::new(vec3(0.0, 1.0, 0.0), vec3(0.0, 0.0, 1.0), vec2(0.0, 0.0)),
            ],
            [
                Material::new(vec3(1.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0)),
                Material::new(vec3(0.0, 0.0, 1.0), vec3(0.0, 0.0, 0.0)),
            ],
            [
                TriangleIdx::new(usizevec3(0, 1, 2), 0),
                TriangleIdx::new(usizevec3(2, 3, 0), 1),
            ],
        );

        let mut triangles = scene.triangles();

        let triangle = triangles.next().unwrap();
        assert_eq!(triangle.vertices()[0].position(), &vec3(1.0, 1.0, 0.0));
        assert_eq!(triangle.vertices()[1].position(), &vec3(1.0, 0.0, 0.0));
        assert_eq!(triangle.vertices()[2].position(), &vec3(0.0, 0.0, 0.0));
        assert_eq!(
            triangle.material(),
            &Material::new(vec3(1.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0))
        );
        assert_eq!(triangle.normal(), vec3(0.0, 0.0, 1.0));

        let triangle = triangles.next().unwrap();
        assert_eq!(triangle.vertices()[0].position(), &vec3(0.0, 0.0, 0.0));
        assert_eq!(triangle.vertices()[1].position(), &vec3(0.0, 1.0, 0.0));
        assert_eq!(triangle.vertices()[2].position(), &vec3(1.0, 1.0, 0.0));
        assert_eq!(
            triangle.material(),
            &Material::new(vec3(0.0, 0.0, 1.0), vec3(0.0, 0.0, 0.0))
        );
        assert_eq!(triangle.normal(), vec3(0.0, 0.0, 1.0));

        assert_eq!(triangles.next(), None);
    }

    #[test]
    fn test_ray_intersection() {
        let scene = Scene::new(
            [
                Vertex::new(vec3(1.0, 1.0, 0.0), vec3(0.0, 0.0, 1.0), vec2(0.0, 0.0)),
                Vertex::new(vec3(0.0, 1.0, 0.0), vec3(0.0, 0.0, 1.0), vec2(0.0, 0.0)),
                Vertex::new(vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0), vec2(0.0, 0.0)),
                Vertex::new(vec3(1.0, 1.0, 1.0), vec3(0.0, 0.0, 1.0), vec2(0.0, 0.0)),
                Vertex::new(vec3(0.0, 1.0, 1.0), vec3(0.0, 0.0, 1.0), vec2(0.0, 0.0)),
                Vertex::new(vec3(0.0, 0.0, 1.0), vec3(0.0, 0.0, 1.0), vec2(0.0, 0.0)),
            ],
            [Material::new(vec3(1.0, 1.0, 1.0), vec3(0.0, 0.0, 0.0))],
            [
                TriangleIdx::new(usizevec3(0, 1, 2), 0),
                TriangleIdx::new(usizevec3(3, 4, 5), 0),
            ],
        );

        let ray = Ray::new(vec3(0.25, 0.25, 2.0), vec3(0.0, 0.0, -1.0));

        let (intersection, triangle) = scene.ray_intersection(&ray).unwrap();

        assert_eq!(intersection.t(), &1.0);
        assert_eq!(intersection.normal(), &vec3(0.0, 0.0, 1.0));
        assert_eq!(intersection.point(), vec3(0.25, 0.25, 1.0));

        assert_eq!(triangle.vertices()[0].position(), &vec3(1.0, 1.0, 1.0));
        assert_eq!(triangle.vertices()[1].position(), &vec3(0.0, 1.0, 1.0));
        assert_eq!(triangle.vertices()[2].position(), &vec3(0.0, 0.0, 1.0));
    }
}
