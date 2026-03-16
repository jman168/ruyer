use std::path::PathBuf;

use clap::Parser;
use glam::u16vec2;
use ruyer::{render::ray_trace, scene::Scene};

/// A simple program for loading a scene from a GLTF file and rendering it.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the GLTF to render.
    #[arg(short, long)]
    scene_path: PathBuf,

    /// Path of the output image.
    #[arg(short, long)]
    image_path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let (scene, camera) = Scene::from_gltf(args.scene_path)?;
    let camera = camera.expect("No camera found in scene!");

    let image = ray_trace(u16vec2(1024, 1024), &camera, &scene);
    image.save(args.image_path)?;

    Ok(())
}
