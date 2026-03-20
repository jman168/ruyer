// pub mod camera;
pub mod geometry;
pub mod object;
// pub mod render;
// pub mod sampling;
// pub mod scene;

// #[cfg(test)]
// mod test {
//     use crate::{
//         camera::PerspectiveCamera,
//         render::ray_trace,
//         scene::{Material, Scene, TriangleIdx},
//     };
//     use glam::{Mat3, Quat, u16vec2, usizevec3, vec2, vec3};
//
//     #[test]
//     fn test_cornell_box() {
//         let scene = Scene::new(
//             [
//                 // Floor
//                 vec3(552.8, 0.0, 0.0),   // front left
//                 vec3(0.0, 0.0, 0.0),     // front right
//                 vec3(0.0, 0.0, 559.2),   // rear right
//                 vec3(549.6, 0.0, 559.2), // rear left
//                 // Ceiling
//                 vec3(556.0, 548.8, 0.0),   // front left
//                 vec3(556.0, 548.8, 559.2), // rear left
//                 vec3(0.0, 548.8, 559.2),   // rear right
//                 vec3(0.0, 548.8, 0.0),     // front right
//             ],
//             [
//                 Material::new(vec3(0.73, 0.73, 0.73)), // White
//                 Material::new(vec3(0.0, 0.481, 0.0)),  // Green
//                 Material::new(vec3(0.657, 0.0, 0.0)),  // Red
//             ],
//             [
//                 // Floor
//                 TriangleIdx::new(usizevec3(0, 1, 2), 0),
//                 TriangleIdx::new(usizevec3(2, 3, 0), 0),
//                 // Ceiling
//                 TriangleIdx::new(usizevec3(4, 5, 6), 0),
//                 TriangleIdx::new(usizevec3(6, 7, 4), 0),
//                 // Back wall
//                 TriangleIdx::new(usizevec3(3, 2, 6), 0),
//                 TriangleIdx::new(usizevec3(6, 5, 3), 0),
//                 // Right wall
//                 TriangleIdx::new(usizevec3(2, 1, 7), 1),
//                 TriangleIdx::new(usizevec3(7, 6, 2), 1),
//                 // Left wall
//                 TriangleIdx::new(usizevec3(0, 3, 5), 2),
//                 TriangleIdx::new(usizevec3(5, 4, 0), 2),
//             ],
//         );
//
//         let cr = Mat3 {
//             x_axis: vec3(-1.0, 0.0, 0.0),
//             y_axis: vec3(0.0, 1.0, 0.0),
//             z_axis: vec3(0.0, 0.0, -1.0),
//         };
//         let cr = Quat::from_mat3(&cr);
//
//         let camera =
//             PerspectiveCamera::new(vec3(278.0, 273.0, -800.0), cr, 0.035, vec2(0.025, 0.025));
//
//         let image = ray_trace(u16vec2(1024, 1024), &camera, &scene);
//         image.save("cornell_box.png").unwrap();
//     }
// }
