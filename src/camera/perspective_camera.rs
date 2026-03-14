use glam::{Quat, U16Vec2, Vec2, Vec3, vec2};

use crate::{camera::Camera, geometry::Ray};

/// Basic perspective camera.
///
/// This cameras up direction is +Y, right direction is +X, and view direction is -Z.
pub struct PerspectiveCamera {
    /// "Eye" of the camera.
    eye: Vec3,
    /// Rotation of the camera.
    rotation: Quat,
    /// Focal length of the camera (distance of the viewing plane from the eye).
    focal_length: f32,
    /// Size of the viewing plane.
    viewing_plane_size: Vec2,
}

impl PerspectiveCamera {
    /// Creates a new perspective camera with the given eye point, rotation, focal length, and
    /// viewing plane size.
    pub fn new(eye: Vec3, rotation: Quat, focal_length: f32, viewing_plane_size: Vec2) -> Self {
        Self {
            eye,
            rotation,
            focal_length,
            viewing_plane_size,
        }
    }
}

impl Camera for PerspectiveCamera {
    fn get_ray(&self, image_size: &U16Vec2, pixel: &U16Vec2) -> Ray {
        // Get the image size and pixel coordinates as floating point.
        let image_size = image_size.as_vec2();
        let pixel = pixel.as_vec2();

        // Get the UV coordinates from -0.5 to 0.5.
        let uv = pixel / (image_size - 1.0) - 0.5;
        // Flip the Y coordinate because positive UV.y points down, not up.
        let uv = uv * vec2(1.0, -1.0);

        // Get the point on the viewing plane.
        let viewing_plane_point = (uv * self.viewing_plane_size).extend(-self.focal_length);

        // Compute the normalized ray direction from the eye point to the point on the viewing
        // plane.
        let ray_direction = self.rotation * viewing_plane_point.normalize();

        Ray::new(self.eye, ray_direction)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use glam::{u16vec2, vec3};

    #[test]
    fn test_get_ray_odd() {
        let camera =
            PerspectiveCamera::new(vec3(0.0, 0.0, 0.0), Quat::IDENTITY, 1.0, vec2(1.0, 1.0));

        assert_eq!(
            camera.get_ray(&u16vec2(11, 11), &u16vec2(5, 5)),
            Ray::new(vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, -1.0))
        );

        assert_eq!(
            camera.get_ray(&u16vec2(11, 11), &u16vec2(0, 0)),
            Ray::new(vec3(0.0, 0.0, 0.0), vec3(-0.5, 0.5, -1.0).normalize())
        );

        assert_eq!(
            camera.get_ray(&u16vec2(11, 11), &u16vec2(10, 10)),
            Ray::new(vec3(0.0, 0.0, 0.0), vec3(0.5, -0.5, -1.0).normalize())
        );

        assert_eq!(
            camera.get_ray(&u16vec2(10, 10), &u16vec2(0, 0)),
            Ray::new(vec3(0.0, 0.0, 0.0), vec3(-0.5, 0.5, -1.0).normalize())
        );

        assert_eq!(
            camera.get_ray(&u16vec2(10, 10), &u16vec2(9, 9)),
            Ray::new(vec3(0.0, 0.0, 0.0), vec3(0.5, -0.5, -1.0).normalize())
        );
    }
}
