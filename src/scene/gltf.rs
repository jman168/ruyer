use glam::{Mat4, Vec2, Vec3, Vec4Swizzles, quat, usizevec3, vec2, vec3};
use gltf::{Document, Node, Result, buffer::Data, camera::Projection, mesh::Mode};
use std::path::Path;

use crate::{
    camera::{Camera, PerspectiveCamera},
    geometry::Vertex,
    scene::{Material, Scene, TriangleIdx},
};

impl Scene {
    /// Loads a scene from a GLTF file.
    pub fn from_gltf(path: impl AsRef<Path>) -> Result<(Self, Option<Box<dyn Camera>>)> {
        let (gltf, buffers, _images) = gltf::import(path)?;

        let mut vertices = vec![];
        let materials = Self::get_materials(&gltf);
        let mut triangles = vec![];

        for scene in gltf.scenes() {
            for node in scene.nodes() {
                Self::process_node(&node, &buffers, &mut vertices, &mut triangles);
            }
        }

        Ok((
            Self::new(vertices, materials, triangles),
            Self::get_camera(&gltf),
        ))
    }

    /// Gets the materials of the document.
    fn get_materials(document: &Document) -> Vec<Material> {
        document
            .materials()
            .map(|mat| {
                let pbr = mat.pbr_metallic_roughness();
                let base_color = pbr.base_color_factor();

                Material::new(vec3(base_color[0], base_color[1], base_color[2]))
            })
            .collect()
    }

    /// Processes a node of the scene adding its vertices and triangles to the scene.
    fn process_node(
        node: &Node,
        buffers: &Vec<Data>,
        vertices: &mut Vec<Vertex>,
        triangles: &mut Vec<TriangleIdx>,
    ) {
        let Some(mesh) = node.mesh() else {
            return;
        };

        let transform = Mat4::from_cols_array_2d(&node.transform().matrix());

        for primitive in mesh.primitives() {
            // We don't support any type of primitive that is not pure triangles.
            if primitive.mode() != Mode::Triangles {
                continue;
            }

            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            // Record the start of this primatives vertices.
            let vertex_offset = vertices.len();

            let positions: Vec<Vec3> = reader
                .read_positions()
                .expect("Mesh primitive has no positions")
                .map(Vec3::from)
                .map(|p| (transform * p.extend(1.0)).xyz())
                .collect();
            let num_positions = positions.len();

            let normals: Vec<Vec3> = reader
                .read_normals()
                .expect("Mesh primitive has no normals")
                .map(Vec3::from)
                .map(|n| (transform * n.extend(0.0)).xyz())
                .collect();

            let uvs: Vec<Vec2> = reader
                .read_tex_coords(0)
                .map(|coords| coords.into_f32().map(Vec2::from).collect())
                .unwrap_or_else(|| vec![vec2(0.0, 0.0); positions.len()]);

            for ((p, n), uv) in positions.into_iter().zip(normals).zip(uvs) {
                vertices.push(Vertex::new(p, n, uv));
            }

            // Get the material index of the mesh.
            let material_index = primitive
                .material()
                .index()
                .expect("Mesh has no material set");

            // If indices are present, use the indices.
            if let Some(indices_reader) = reader.read_indices() {
                let indices: Vec<u32> = indices_reader.into_u32().collect();
                for chunk in indices.chunks_exact(3) {
                    triangles.push(TriangleIdx::new(
                        usizevec3(
                            vertex_offset + chunk[0] as usize,
                            vertex_offset + chunk[1] as usize,
                            vertex_offset + chunk[2] as usize,
                        ),
                        material_index,
                    ));
                }
            }
            // If indices are not present, the vertices are already in triangle order.
            else {
                for i in (0..num_positions).step_by(3) {
                    triangles.push(TriangleIdx::new(
                        usizevec3(
                            vertex_offset + i,
                            vertex_offset + i + 1,
                            vertex_offset + i + 2,
                        ),
                        material_index,
                    ));
                }
            }
        }

        // Process children of this node.
        for child in node.children() {
            Self::process_node(&child, buffers, vertices, triangles);
        }
    }

    /// Returns the first perspective camera in the document if it exists.
    fn get_camera(document: &Document) -> Option<Box<dyn Camera>> {
        for node in document.nodes() {
            let Some(camera) = node.camera() else {
                continue;
            };

            let Projection::Perspective(perspective) = camera.projection() else {
                continue;
            };

            let transform = node.transform();
            let (translation, rotation, _scale) = transform.decomposed();

            let viewing_plane_height = (perspective.yfov() / 2.0).tan();
            let viewing_plane_width =
                viewing_plane_height * perspective.aspect_ratio().unwrap_or(1.0);

            return Some(Box::new(PerspectiveCamera::new(
                vec3(translation[0], translation[1], translation[2]),
                quat(rotation[0], rotation[1], rotation[2], rotation[3]),
                0.5,
                vec2(viewing_plane_width, viewing_plane_height),
            )));
        }

        None
    }
}
