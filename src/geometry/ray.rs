use glam::Vec3;

/// Ray with an origin and direction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    /// Create a new ray with a given origin and direction.
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Returns the origin of the ray.
    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    /// Returns the direction of the ray.
    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    /// Returns a point along the ray at a given t.
    pub fn point_at(&self, t: &f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

/// Intersection with parameter t, normal, and ray.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RayIntersection<'a> {
    t: f32,
    normal: Vec3,
    ray: &'a Ray,
}

impl<'a> RayIntersection<'a> {
    /// Creates a new intersection with a given t, normal, and ray.
    pub fn new(t: f32, normal: Vec3, ray: &'a Ray) -> Self {
        Self { t, normal, ray }
    }

    /// Returns the t parameter of the intersection.
    pub fn t(&self) -> &f32 {
        &self.t
    }

    /// Returns the normal of the intersection.
    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    /// Returns the point of intersection.
    pub fn point(&self) -> Vec3 {
        self.ray.point_at(&self.t)
    }
}
