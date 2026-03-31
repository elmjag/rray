use crate::{camera::Camera, mesh::Mesh};
use serde::Deserialize;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct Scene {
    camera: _Camera,
    meshes: Vec<Mesh>,
}

#[derive(Debug, Deserialize)]
struct _Camera {
    width: u32,
    height: u32,
    depth: f32,
}

fn open_file(file: PathBuf) -> Result<File, String> {
    match File::open(file) {
        Ok(f) => Ok(f),
        Err(err) => Err(String::from(format!("{err}"))),
    }
}

fn get_mesh(scene: &mut Scene) -> Mesh {
    let meshes = &mut scene.meshes;
    if meshes.len() != 1 {
        todo!("loading multiple meshes");
    }
    meshes.pop().unwrap()
}

fn get_camera(scene: &Scene) -> Camera {
    let cam = &scene.camera;
    Camera::new(cam.width, cam.height, cam.depth)
}

pub fn load_scene(file: PathBuf) -> Result<(Camera, Mesh), String> {
    let scene: Result<Scene, _> = serde_saphyr::from_reader(open_file(file)?);

    match scene {
        Ok(mut scene) => Ok((get_camera(&scene), get_mesh(&mut scene))),
        Err(err) => Err(String::from(format!("YAML parse error {}", err.render()))),
    }
}
