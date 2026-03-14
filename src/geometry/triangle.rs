use glam::Vec3;

use crate::geometry::{Ray, RayIntersection};

/// Trait for representing triangles.
pub trait Triangle {
    /// Returns the three vertices of the triangle.
    fn vertices(&self) -> [&Vec3; 3];

    /// Returns the normal of the triangle assuming the triangle vertices are counter clockwise
    /// oriented.
    fn normal(&self) -> Vec3 {
        let v = self.vertices();

        let e1 = v[1] - v[0];
        let e2 = v[2] - v[0];
        e1.cross(e2).normalize()
    }

    /// Returns [`RayIntersection`] of the ray equation if there was an intersection with the triangle.
    ///
    /// The default implementation uses the [Möller–Trumbore intersection algorithm](https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm).
    ///
    /// NOTE: This algorithm DOES perform backface culling assuming CCW-wound triangles.
    fn ray_intersection<'a>(&self, ray: &'a Ray) -> Option<RayIntersection<'a>> {
        let vertices = self.vertices();

        let edge1 = vertices[1] - vertices[0];
        let edge2 = vertices[2] - vertices[0];

        // Backface culling, assuming CCW-wound triangles.
        let normal = edge1.cross(edge2); // no need to normalize
        if normal.dot(*ray.direction()) > 0.0 {
            return None;
        }

        let ray_cross_e2 = ray.direction().cross(edge2);
        let det = edge1.dot(ray_cross_e2);

        // Ray is parallel to triangle
        if det.abs() < f32::EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let s = ray.origin() - vertices[0];
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
            Some(RayIntersection::new(t, normal.normalize(), ray))
        }
        // This means that there is a line intersection but not a ray intersection.
        else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use glam::vec3;

    struct TestTriangle {
        vertices: [Vec3; 3],
    }

    impl TestTriangle {
        fn new(vertices: [Vec3; 3]) -> Self {
            Self { vertices }
        }
    }

    impl Triangle for TestTriangle {
        fn vertices(&self) -> [&Vec3; 3] {
            self.vertices.each_ref()
        }
    }

    #[test]
    fn test_normal() {
        let triangle = TestTriangle::new([
            vec3(1.0, 1.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            vec3(0.0, 0.0, 0.0),
        ]);

        assert_eq!(triangle.normal(), vec3(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_ray_intersection() {
        let triangle = TestTriangle::new([
            vec3(1.0, 1.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            vec3(0.0, 0.0, 0.0),
        ]);

        let ray = Ray::new(vec3(0.25, 0.25, 1.0), vec3(0.0, 0.0, -1.0));

        let intersection = triangle.ray_intersection(&ray).unwrap();

        assert_eq!(intersection.t(), &1.0);
        assert_eq!(intersection.normal(), &vec3(0.0, 0.0, 1.0));
        assert_eq!(intersection.point(), vec3(0.25, 0.25, 0.0));
    }
}
