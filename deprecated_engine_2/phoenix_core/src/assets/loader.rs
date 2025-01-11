use gltf::Gltf;
use std::path::Path;
use std::fs::File;
use std::io::Read;

use crate::graphics::mesh::*;

pub struct AssetLoader;

impl AssetLoader {
  pub fn new() -> AssetLoader {
    AssetLoader
  }

  pub fn load_gltf(&self, path: &str) -> Result<Gltf, String> {
    let file_contents = std::fs::read(path).map_err(|e| e.to_string())?;
    Gltf::from_slice(&file_contents).map_err(|e| e.to_string())
  }

  pub fn process_gltf_meshes(&self, gltf: Gltf) -> Vec<StaticMesh> {
    let mut static_meshes = Vec::new();

    for mesh in gltf.meshes() {
      for primitive in mesh.primitives() {
        let reader = primitive.reader(|buffer| Some(&gltf.buffers()[buffer.index()].data));

        // Read vertex positions
        let positions: Vec<[f32; 3]> = reader
          .read_positions()
          .expect("Mesh must have positions")
          .collect();

        // Read normals (optional)
        let normals: Vec<[f32; 3]> = reader
          .read_normals()
          .unwrap_or_else(|| vec![[0.0, 0.0, 0.0]; positions.len()])
          .collect();

        // Read texture coordinates (optional)
        let tex_coords: Vec<[f32; 2]> = reader
          .read_tex_coords(0)
          .map(|tc| tc.into_f32().collect())
          .unwrap_or_else(|| vec![[0.0, 0.0]; positions.len()]);

        // Combine vertex data
        let vertices: Vec<Vertex> = positions
          .into_iter()
          .zip(normals.into_iter())
          .zip(tex_coords.into_iter())
          .map(|((position, normal), texture_coordinates )| Vertex {
          position: position.into(),
          normal: normal.into(),
          texture_coordinates: texture_coordinates.into(),
          })
          .collect();

        // Read indices
        let indices: Vec<u32> = reader
          .read_indices()
          .map(|indices| indices.into_u32().collect())
          .unwrap_or_else(|| (0..vertices.len() as u32).collect());

        // Load textures (dummy for now; expand later)
        let textures = vec![];

        // Create the StaticMesh
        let static_mesh = StaticMesh::new(vertices, indices, textures);

        static_meshes.push(static_mesh);
      }
    }

    static_meshes
  }

}
