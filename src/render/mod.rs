//! Rendering module.

use glam::{U16Vec2, u16vec2};
use image::{ImageBuffer, Rgb, RgbImage};

use crate::{camera::Camera, scene::Scene};

/// Ray trace a scene.
pub fn ray_trace(
    image_size: U16Vec2,
    camera: &impl Camera,
    scene: &Scene,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img = RgbImage::new(image_size.x.into(), image_size.y.into());

    for i in 0..image_size.x {
        for j in 0..image_size.y {
            let ray = camera.get_ray(&image_size, &u16vec2(i, j));

            if let Some((_, triangle)) = scene.ray_intersection(&ray) {
                let color = (triangle.material().color() * 255.0).as_u8vec3();
                img.put_pixel(i.into(), j.into(), Rgb([color.x, color.y, color.z]));
            }
        }
    }

    img
}
