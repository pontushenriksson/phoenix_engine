use gl;
use crate::ecs::components::*;
use crate::graphics::renderer::*;

use super::renderer::{ElementBufferObject, VertexArrayObject, VertexBufferObject};

pub struct Vertex {
  pub position: cgmath::Vector3<f32>,
  pub normal: cgmath::Vector3<f32>,
  pub texture_coordinates: cgmath::Vector2<f32>,
}

pub struct StaticMesh {
  pub vertices: Vec<Vertex>,
  pub indices: Vec<u32>,
  pub textures: Vec<Texture2D>,
  vao: VertexArrayObject,
  vbo: VertexBufferObject,
  ebo: ElementBufferObject,
}

impl StaticMesh {
  pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture2D>) -> StaticMesh {
    let vao = VertexArrayObject::new();
    let vbo = VertexBufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    let ebo = ElementBufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);

    /*
    
    vao.bind();
    vbo.bind();
    vbo.upload_data(&vertices, gl::STATIC_DRAW);
    ebo.bind();
    ebo.upload_data(&indices, gl::STATIC_DRAW);
    
    */

    StaticMesh {
      vertices,
      indices,
      textures,
      vao,
      vbo,
      ebo,
    }
  }
}
