use std::sync::Arc;

use gltf::Gltf;
use tokio::fs;
use async_trait::async_trait;
use futures::future::join_all;

use crate::graphics::mesh::StaticMesh;
use crate::graphics::texture::{Texture, TextureType};

/// Stores loaded vertex data.
#[derive(Debug)]
pub struct RawVertexData {
  pub data: Vec<f32>, // Interleaved vertex data
  pub stride: usize,
}

impl RawVertexData {
  pub fn new(data: Vec<f32>, stride: usize) -> Self {
    assert!(stride > 0, "Stride must be greater than 0.");
    assert!(data.len() % stride == 0, "Data length must be a multiple of stride.");
    Self { data, stride }
  }

  /// Extract positions (first 3 floats per vertex)
  pub fn positions(&self) -> Vec<[f32; 3]> {
    self.data.chunks(self.stride).map(|chunk| {
      [chunk[0], chunk[1], chunk[2]] // Assuming positions are the first 3 floats
    }).collect()
  }

  /// Extract normals (next 3 floats per vertex after positions)
  pub fn normals(&self) -> Vec<[f32; 3]> {
    self.data.chunks(self.stride).map(|chunk| {
      [chunk[3], chunk[4], chunk[5]] // Assuming normals follow positions
    }).collect()
  }

  /// Extract texture coordinates (next 2 floats after normals)
  pub fn tex_coords(&self) -> Vec<[f32; 2]> {
    self.data.chunks(self.stride).map(|chunk| {
      [chunk[6], chunk[7]] // Assuming tex coords follow normals
    }).collect()
  }
}

#[async_trait]
pub trait Asset: Sized {
  async fn load(
    path: &str,
    target: gl::types::GLenum,
    usage: gl::types::GLenum,
    ebo_type: gl::types::GLenum
  ) -> Self;
}

pub struct AssetLoader;

impl AssetLoader {
  pub async fn load<T: Asset + Send>(
    path: &str,
    target: gl::types::GLenum,
    usage: gl::types::GLenum,
    ebo_type: gl::types::GLenum
  ) -> T {
    T::load(path, target, usage, ebo_type).await
  }
}

pub async fn load_gltf(
  path: &str,
) -> Box<StaticMesh> {
  let gltf_data = fs::read(path).await.expect("Failed to read glTF file");
  let gltf = Arc::new(Gltf::from_slice(&gltf_data).expect("Failed to parse glTF file"));

  let buffers = join_all(
      gltf.buffers().map(|buffer| {
          let gltf = Arc::clone(&gltf);
          async move {
              match buffer.source() {
                  gltf::buffer::Source::Bin => {
                      gltf.blob
                          .as_ref()
                          .expect("No binary blob found for Bin buffer")
                          .to_vec()
                  }
                  gltf::buffer::Source::Uri(uri) => {
                      fs::read(uri).await.expect("Failed to read buffer from URI")
                  }
              }
          }
      }),
  )
  .await;

  for mesh in gltf.meshes() {
      for primitive in mesh.primitives() {
          let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
          let positions: Vec<[f32; 3]> = reader.read_positions().unwrap().collect();
          let normals: Vec<[f32; 3]> = reader.read_normals().map_or_else(Vec::new, |iter| iter.collect());
          let tex_coords: Vec<[f32; 2]> = reader
              .read_tex_coords(0)
              .map_or_else(Vec::new, |t| t.into_f32().collect());
          let indices: Vec<u32> = reader.read_indices().unwrap().into_u32().collect();

          let mut vertex_data = Vec::new();
          for (i, pos) in positions.iter().enumerate() {
              vertex_data.extend_from_slice(pos);
              if normals.len() > 0 {
                  vertex_data.extend_from_slice(&normals[i]);
              } else {
                  vertex_data.extend_from_slice(&[0.0, 0.0, 0.0]);
              }
              if tex_coords.len() > 0 {
                  vertex_data.extend_from_slice(&tex_coords[i]);
              } else {
                  vertex_data.extend_from_slice(&[0.0, 0.0]);
              }
          }

          let stride = 8; // 3 for position + 3 for normals + 2 for tex coords
          let raw_vertex_data = RawVertexData::new(vertex_data, stride);

          // Create the StaticMesh with the new `Mesh` structure
          return StaticMesh::new(raw_vertex_data, Some(indices));
      }
  }
  panic!("No valid StaticMesh data found");
}

pub fn load_textures_from_gltf(
  primitive: gltf::Primitive,
  buffers: &[gltf::buffer::Data], // Buffer data from gltf::import
) -> Vec<Texture> {
  let material = primitive.material();
  let mut diffuse_maps = Vec::new();

  if let Some(base_color_texture) = material.pbr_metallic_roughness().base_color_texture() {
      let texture = base_color_texture.texture();

      if let gltf::image::Source::View { view, mime_type: _ } = texture.source().source() {
          // Extract the raw data for the texture
          let buffer = &buffers[view.buffer().index()];
          let offset = view.offset();
          let length = view.length();
          let texture_data = &buffer[offset..offset + length];

          // Pass raw data to Texture::new_from_gltf
          let diffuse_texture = Texture::new_from_gltf(
              texture_data,
              TextureType::Diffuse, // Specify texture type
          );
          diffuse_maps.push(diffuse_texture);
      }
  }

  diffuse_maps
}

