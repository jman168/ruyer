use criterion::{Criterion, criterion_group, criterion_main};

use glam::{Quat, mat3, u16vec2, usizevec3, vec2, vec3};
use ruyer::{
    camera::PerspectiveCamera,
    render::ray_trace,
    scene::{Material, Scene, TriangleIdx},
};

fn criterion_benchmark(c: &mut Criterion) {
    let scene = Scene::new(
        [
            // Floor
            vec3(552.8, 0.0, 0.0),   // front left
            vec3(0.0, 0.0, 0.0),     // front right
            vec3(0.0, 0.0, 559.2),   // rear right
            vec3(549.6, 0.0, 559.2), // rear left
            // Ceiling
            vec3(556.0, 548.8, 0.0),   // front left
            vec3(556.0, 548.8, 559.2), // rear left
            vec3(0.0, 548.8, 559.2),   // rear right
            vec3(0.0, 548.8, 0.0),     // front right
        ],
        [
            Material::new(vec3(0.73, 0.73, 0.73)), // White
            Material::new(vec3(0.0, 0.481, 0.0)),  // Green
            Material::new(vec3(0.657, 0.0, 0.0)),  // Red
        ],
        [
            // Floor
            TriangleIdx::new(usizevec3(0, 1, 2), 0),
            TriangleIdx::new(usizevec3(2, 3, 0), 0),
            // Ceiling
            TriangleIdx::new(usizevec3(4, 5, 6), 0),
            TriangleIdx::new(usizevec3(6, 7, 4), 0),
            // Back wall
            TriangleIdx::new(usizevec3(3, 2, 6), 0),
            TriangleIdx::new(usizevec3(6, 5, 3), 0),
            // Right wall
            TriangleIdx::new(usizevec3(2, 1, 7), 1),
            TriangleIdx::new(usizevec3(7, 6, 2), 1),
            // Left wall
            TriangleIdx::new(usizevec3(0, 3, 5), 2),
            TriangleIdx::new(usizevec3(5, 4, 0), 2),
        ],
    );

    let cr = mat3(
        vec3(-1.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        vec3(0.0, 0.0, -1.0),
    );
    let cr = Quat::from_mat3(&cr);

    let camera = PerspectiveCamera::new(vec3(278.0, 273.0, -800.0), cr, 0.035, vec2(0.025, 0.025));

    c.bench_function("256x256 Cornell Box", |b| {
        b.iter(|| ray_trace(u16vec2(256, 256), &camera, &scene))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
