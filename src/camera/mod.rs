//! Structures and traits relating to cameras.

mod perspective_camera;
pub use perspective_camera::PerspectiveCamera;

use crate::geometry::Ray;

use glam::U16Vec2;

/// Trait for representing a camera.
pub trait Camera {
    /// Gets a ray for a given pixel and given image size.
    fn get_ray(&self, image_size: &U16Vec2, pixel: &U16Vec2) -> Ray;
}

impl Camera for Box<dyn Camera> {
    fn get_ray(&self, image_size: &U16Vec2, pixel: &U16Vec2) -> Ray {
        (**self).get_ray(image_size, pixel)
    }
}
