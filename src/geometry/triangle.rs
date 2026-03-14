use glam::Vec3;

/// Trait for representing triangles.
pub trait Triangle {
    /// Returns the three vertices of the triangle.
    fn vertices(&self) -> [&Vec3; 3];

    /// Returns the normal of the triangle assuming the triangle vertices are counter clockwise
    /// oriented.
    fn normal(&self) -> Vec3 {
        let v = self.vertices();
        (v[1] - v[0]).cross(v[2] - v[0]).normalize()
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
}
