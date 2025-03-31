use crate::gl_call;
use crate::debugger::debugger::Debugger;

use crate::graphics::data::RenderDataPrimitive;
use crate::graphics::data::{ElementBufferObject, VertexArrayObject, VertexBufferObject};
use super::data::VertexDescriptor;

pub enum BufferType {
  Static,
  Dynamic,
  Stream,
}

impl BufferType {
  pub fn as_gl(&self) -> gl::types::GLenum {
    match self {
      BufferType::Static => gl::STATIC_DRAW,
      BufferType::Dynamic => gl::DYNAMIC_DRAW,
      BufferType::Stream => gl::STREAM_DRAW,
    }
  }
}

#[derive(Clone)]
pub struct Mesh<T: RenderDataPrimitive, U: RenderDataPrimitive> {
  // Render data
  vao: VertexArrayObject,
  vbo: VertexBufferObject,
  ebo: Option<ElementBufferObject>,
  descriptor: VertexDescriptor,

  // Raw data
  vertices: Vec<T>,
  indices: Option<Vec<U>>,
}

impl<T, U> Mesh<T, U>
where T: RenderDataPrimitive, U: RenderDataPrimitive {
  pub fn new(
    usage: BufferType,
    vertices: Vec<T>,
    indices: Option<Vec<U>>,
    descriptor: VertexDescriptor,
  ) -> Mesh<T, U> {
    let vao = VertexArrayObject::new();
    vao.bind();

    let vbo = VertexBufferObject::new(usage.as_gl());
    vbo.bind();
    vbo.store(&T::to_f32_vec(&vertices));

    let mut ebo: Option<ElementBufferObject> = None;

    if let Some(ref indices_vec) = indices {
      let element_buffer = ElementBufferObject::new(usage.as_gl());
      element_buffer.bind();
      element_buffer.store(&U::to_u32_vec(indices_vec)); // Borrow instead of move
      element_buffer.bind();
      ebo = Some(element_buffer);
    }

    let mut offset = 0;
    for (index, attribute) in descriptor.attributes.iter().enumerate() {
      unsafe {
        gl_call!(gl::EnableVertexAttribArray(index as u32));
        gl_call!(gl::VertexAttribPointer(
          index as u32,
          attribute.size(),
          attribute.gl_type(),
          attribute.normalized(),
          descriptor.stride * std::mem::size_of::<T>() as gl::types::GLsizei,
          offset as *const std::ffi::c_void,
        ));
      }

      offset += attribute.size() * std::mem::size_of::<gl::types::GLfloat>() as i32;
    }

    VertexArrayObject::unbind();
    VertexBufferObject::unbind();
    ElementBufferObject::unbind();

    Mesh {
      vao,
      vbo,
      ebo,
      descriptor,
      vertices,
      indices,
    }
  }

  pub fn draw(&self) {
    self.vao.bind();
    if let Some(indices) = &self.indices {
      unsafe {
        gl_call!(gl::DrawElements(gl::TRIANGLES, indices.len() as i32, gl::UNSIGNED_INT, std::ptr::null()));
      }
    } else {
      unsafe {
        gl_call!(gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32 / self.descriptor.stride));
      }
    }
    VertexArrayObject::unbind();
  }
}

impl<T: RenderDataPrimitive, U: RenderDataPrimitive> Drop for Mesh<T, U> {
  fn drop(&mut self) {
    self.vbo.delete();
    
    if let Some(ebo) = &mut self.ebo {
      ebo.delete();
    }
    
    self.vao.delete();
  }
}
