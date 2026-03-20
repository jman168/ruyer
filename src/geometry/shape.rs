use crate::geometry::{Ray, RayIntersection};

/// A trait for representing abstract geometric shapes.
pub trait Shape<S> {
    /// Returns [`RayIntersection`] of the ray equation if there was an intersection with the shape.
    fn ray_intersection<'a>(&self, ray: &'a Ray, scene: &S) -> Option<RayIntersection<'a>>;
}
