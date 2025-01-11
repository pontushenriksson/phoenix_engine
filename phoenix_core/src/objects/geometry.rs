use cgmath::SquareMatrix;

use crate::graphics::data::Attribute;
use crate::graphics::mesh::{Mesh, StaticMesh};
use crate::graphics::texture::Sampler;

pub struct Quad {
  pub mesh: StaticMesh,
  pub matrix: cgmath::Matrix4<f32>,
}

impl Quad {
  pub fn new(sampler: Option<Box<dyn Sampler>>) -> Quad {
    let vertices: [gl::types::GLfloat; 32] = [
    // Positions              Color                   Texture Coords
      -0.5, -0.5,  0.0,       1.0, 1.0, 1.0,          0.0, 0.0,
       0.5, -0.5,  0.0,       1.0, 0.0, 0.0,          1.0, 0.0,
       0.5,  0.5,  0.0,       0.0, 1.0, 0.0,          1.0, 1.0,
      -0.5,  0.5,  0.0,       0.0, 0.0, 1.0,          0.0, 1.0,
    ];

    let indices: [gl::types::GLuint; 6] = [
      0, 1, 2,
      2, 3, 0
    ];

    let attributes: [Attribute; 3] = [
      Attribute::Vec3,
      Attribute::Vec3,
      Attribute::Vec2,
    ];

    let mesh = StaticMesh::new(
      vertices.to_vec(),
      indices.to_vec(),
      &attributes,
      Some(vec![sampler.unwrap()])
    );

    Quad {
      mesh,
      matrix: cgmath::Matrix4::identity(),
    }
  }

  pub fn indices() -> u32 {
    6
  }
}
