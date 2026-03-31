use crate::{camera::Camera, mesh::Mesh};
use std::path::PathBuf;

mod default;
mod reader;

pub fn load_scene(scene: Option<PathBuf>) -> Result<(Camera, Mesh), String> {
    match scene {
        Some(file) => Ok(reader::load_scene(file)?),
        None => Ok((default::get_camera(), default::get_mesh())),
    }
}
