use std::collections::HashMap;

use crate::assets::loader::RawVertexData;
use crate::graphics::data::RenderData;
use crate::graphics::shader::ShaderProgram;

use crate::graphics::data::{
  VertexArrayObject,
  VertexBufferObject,
  ElementBufferObject,
  VertexAttributeDescriptor,
  VertexAttribute
};

pub enum MaterialType {
  Basic,  // Simple unlit material,
  Pbr,    // Physically-Based Rendering
  Custom, // Custom shaders provided by the user
}

pub struct UniformMatrices {
  pub model: cgmath::Matrix4<f32>,
  pub view: cgmath::Matrix4<f32>,
  pub projection: cgmath::Matrix4<f32>
}

pub enum UniformValue {
  Float(f32),
  Vec3(cgmath::Vector3<f32>),
  Mat4(cgmath::Matrix4<f32>),
  // Texture(Texture),
}

pub struct  UniformCollection {
  pub matrices: UniformMatrices,
  pub other: HashMap<String, UniformValue>,
}

pub struct Material {
  pub r#type: MaterialType,
  pub shader: ShaderProgram,
  pub uniforms: UniformCollection,
}

pub struct Mesh {
  pub vertices: RawVertexData,
  pub indices: Option<Vec<u32>>,
  pub render_data: RenderData,
}

impl Mesh {
  pub fn new(
    vertices: RawVertexData,
    indices: Option<Vec<u32>>,
  ) -> Mesh {
    // Step 1: Create RenderData
    let vao = VertexArrayObject::new();
    let vbo = VertexBufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    let mut ebo = indices.as_ref().map(|_| ElementBufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW));

    vao.bind();
    vbo.bind();

    // Store vertex data into VBO
    vbo.store_data(&vertices.data); // Assuming positions in RawVertexData

    // Store indices in EBO (if present)
    if let Some(ref indices_data) = indices {
      if let Some(ref mut ebo_obj) = ebo {
        ebo_obj.bind();
        ebo_obj.store_data(&indices_data);
      }
    }

    // Define vertex attribute pointers (Position, TexCoord, Normals, etc.)
    // Assumes positions and texcoords in interleaved layout
    let stride = (vertices.stride * std::mem::size_of::<f32>()) as gl::types::GLsizei;

    let position_attribute = VertexAttributeDescriptor {
      location: 0, // Location in the shader
      size: 3,     // x, y, z
      data_type: gl::FLOAT,
      normalized: gl::FALSE,
      stride,
      offset: 0,
    };

    let texcoord_attribute = VertexAttributeDescriptor {
      location: 1, // Texture coordinate location
      size: 2,     // u, v
      data_type: gl::FLOAT,
      normalized: gl::FALSE,
      stride,
      offset: (3 * std::mem::size_of::<f32>()),
    };

    // Enable and set up attributes
    let vap = VertexAttribute::from(&position_attribute);
    vap.enable();

    let vap_tex = VertexAttribute::from(&texcoord_attribute);
    vap_tex.enable();

    vao.unbind();
    vbo.unbind();
    if let Some(ref ebo_obj) = ebo {
        ebo_obj.unbind();
    }

    // Prepare the RenderData (VAO, VBO, EBO)
    let render_data = RenderData {
      vao,
      vbo,
      ebo,
    };

  // Return Mesh with data
    Mesh {
      vertices,
      indices,
      render_data,
    }
  }
}
