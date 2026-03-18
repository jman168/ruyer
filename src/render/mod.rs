//! Rendering module.

use glam::{Mat3, U16Vec2, Vec3, mat3, u16vec2, vec3};
use image::{ImageBuffer, Rgb, RgbImage};
use rand::RngExt;

use crate::{
    camera::Camera,
    geometry::{Ray, RayIntersection},
    sampling::CosineWeightedUnitHemisphere,
    scene::{Material, Scene},
};

/// Generates a transform matrix such that the z axis points in the same direction as n and the x
/// and y axis are tangent and bitangent.
///
/// This is an implementation of Frisvad's method.
fn frisvad(n: &Vec3) -> Mat3 {
    if n.z < -0.9999999
    // Handle the singularity
    {
        return mat3(vec3(0.0, -1.0, 0.0), vec3(-1.0, 0.0, 0.0), *n);
    }

    let a = 1.0 / (1.0 + n.z);
    let b = -n.x * n.y * a;

    mat3(
        vec3(1.0 - n.x * n.x * a, b, -n.x),
        vec3(b, 1.0 - n.y * n.y * a, -n.y),
        *n,
    )
}

// L_r function of the rendering equation.
#[allow(non_snake_case)]
fn Lr(scene: &Scene, intersection: &RayIntersection, material: &Material, depth: u8) -> Vec3 {
    let normal = intersection.normal();
    let w_i = frisvad(normal) * rand::rng().sample(CosineWeightedUnitHemisphere::default());

    let ray = Ray::new(intersection.point() + normal * 1.0e-3, w_i);

    material.color() * Li(scene, &ray, depth - 1)
}

// L_i function of the rendering equation.
#[allow(non_snake_case)]
fn Li(scene: &Scene, ray: &Ray, depth: u8) -> Vec3 {
    // If we have hit our maximum depth, return black.
    if depth == 0 {
        return vec3(0.0, 0.0, 0.0);
    }

    // If there is no intersection, return black.
    let Some((intersection, triangle)) = scene.ray_intersection(ray) else {
        return vec3(0.0, 0.0, 0.0);
    };

    let material = triangle.material();

    let Le = material.emission();
    let Lr = Lr(scene, &intersection, material, depth);

    Le + Lr
}

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
            let mut color = Vec3::ZERO;

            for _ in 0..256 {
                color += Li(scene, &ray, 10);
            }

            let color = (color * 255.0 / 256.0).as_u8vec3();
            img.put_pixel(i.into(), j.into(), Rgb([color.x, color.y, color.z]));
        }
    }

    img
}
