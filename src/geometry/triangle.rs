use crate::geometry::{Ray, RayIntersection, Shape, Vertex, VertexIdx};
use glam::Vec3;
use std::ops::Index;

/// Structure for storing a single triangle.
pub struct Triangle {
    vertices: [VertexIdx; 3],
}

impl Triangle {
    /// Creates a new triangle given three vertex indices.
    pub fn new(vertices: [VertexIdx; 3]) -> Self {
        Self { vertices }
    }

    /// Returns the normal of the triangle.
    pub fn normal<S>(&self, scene: &S) -> Vec3
    where
        S: Index<VertexIdx, Output = Vertex>,
    {
        self.vertices
            .iter()
            .map(|idx| scene[*idx].normal())
            .sum::<Vec3>()
            / 3.0
    }
}

impl<S> Shape<S> for Triangle
where
    S: Index<VertexIdx, Output = Vertex>,
{
    fn ray_intersection<'a>(&self, ray: &'a Ray, scene: &S) -> Option<RayIntersection<'a>> {
        let edge1 = scene[self.vertices[1]].position() - scene[self.vertices[0]].position();
        let edge2 = scene[self.vertices[2]].position() - scene[self.vertices[0]].position();

        let ray_cross_e2 = ray.direction().cross(edge2);
        let det = edge1.dot(ray_cross_e2);

        // Ray is parallel to triangle
        if det.abs() < f32::EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let s = ray.origin() - scene[self.vertices[0]].position();
        let u = inv_det * s.dot(ray_cross_e2);

        // Ray passes outside edge2's bounds
        if u < -f32::EPSILON || u - 1.0 > f32::EPSILON {
            return None;
        }

        let s_cross_e1 = s.cross(edge1);
        let v = inv_det * ray.direction().dot(s_cross_e1);

        // Ray passes outside edge1's bounds
        if v < -f32::EPSILON || u + v - 1.0 > f32::EPSILON {
            return None;
        }

        // The ray line intersects with the triangle.
        // We compute t to find where on the ray the intersection is.
        let t = inv_det * edge2.dot(s_cross_e1);

        // Ray intersection
        if t > f32::EPSILON {
            Some(RayIntersection::new(t, self.normal(scene), ray))
        }
        // This means that there is a line intersection but not a ray intersection.
        else {
            None
        }
    }
}

/// A struct used for accessing a triangle by its index in some container.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TriangleIdx(pub u32);

#[cfg(test)]
impl Index<TriangleIdx> for Vec<Triangle> {
    type Output = Triangle;

    fn index(&self, index: TriangleIdx) -> &Self::Output {
        &self[index.0 as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use glam::{vec2, vec3};

    fn test_vertices() -> Vec<Vertex> {
        vec![
            Vertex::new(vec3(1.0, 1.0, 0.0), vec3(0.0, 0.0, 1.0), vec2(0.0, 0.0)),
            Vertex::new(vec3(0.0, 1.0, 0.0), vec3(0.0, 0.0, 1.0), vec2(0.0, 0.0)),
            Vertex::new(vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0), vec2(0.0, 0.0)),
        ]
    }

    #[test]
    fn test_normal() {
        let vertices = test_vertices();
        let triangle = Triangle::new([VertexIdx(0), VertexIdx(1), VertexIdx(2)]);

        assert_eq!(triangle.normal(&vertices), vec3(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_ray_intersection() {
        let vertices = test_vertices();
        let triangle = Triangle::new([VertexIdx(0), VertexIdx(1), VertexIdx(2)]);

        let ray = Ray::new(vec3(0.25, 0.25, 1.0), vec3(0.0, 0.0, -1.0));

        let intersection = triangle.ray_intersection(&ray, &vertices).unwrap();

        assert_eq!(intersection.t(), &1.0);
        assert_eq!(intersection.normal(), &vec3(0.0, 0.0, 1.0));
        assert_eq!(intersection.point(), vec3(0.25, 0.25, 0.0));

        let ray = Ray::new(vec3(0.25, 0.25, 1.0), vec3(0.0, 0.0, 1.0));

        let intersection = triangle.ray_intersection(&ray, &vertices);
        assert_eq!(intersection, None);
    }
}
