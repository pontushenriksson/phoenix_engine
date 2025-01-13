use cgmath::SquareMatrix;

use crate::graphics::data::{Attribute, VertexDescriptor};
use crate::graphics::mesh::{BufferType, Mesh};

pub struct Quad {
  pub mesh: Mesh<gl::types::GLfloat, gl::types::GLuint>,
  pub matrix: cgmath::Matrix4<f32>,
}

impl Quad {
  pub fn new() -> Quad {
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

    let descriptor = VertexDescriptor {
      attributes: vec![
        Attribute::Vec3,
        Attribute::Vec3,
        Attribute::Vec2,
      ],
      stride: 8
    };

    let mesh = Mesh::new(
      BufferType::Static,
      vertices.to_vec(),
      Some(indices.to_vec()),
      descriptor,
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
