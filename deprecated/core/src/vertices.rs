use gl;
use std::ffi::c_void;
use std::mem;

pub struct VertexArrayObject {
  pub id: gl::types::GLuint,
}

impl VertexArrayObject {
  pub fn new() -> VertexArrayObject {
    let mut id: u32 = 0;
    unsafe {
      gl::CreateVertexArrays(1, &mut id);
    }

    VertexArrayObject { id }
  }

  pub fn bind(&self) {
    unsafe {
      gl::BindVertexArray(self.id);
    }
  }

  pub fn unbind(&self) {
    unsafe {
      gl::BindVertexArray(0);
    }
  }
}

pub struct VertexBufferObject {
  pub id: gl::types::GLuint,
  pub r#type: gl::types::GLenum,
  pub usage: gl::types::GLenum,
}

impl VertexBufferObject {
  pub fn new (r#type: gl::types::GLenum, usage: gl::types::GLenum) -> VertexBufferObject {
    let mut id: u32 = 0;
    unsafe {
      gl::CreateBuffers(1, &mut id);
    }

    VertexBufferObject { id, r#type, usage }
  }

  pub fn bind(&self) {
    unsafe {
      gl::BindBuffer(self.r#type, self.id);
    }
  }

  pub fn unbind(&self) {
    unsafe {
      gl::BindBuffer(self.r#type, 0);
    } 
  }

  pub fn store_f32_data(&self, data: &[f32]) {
    unsafe {
      gl::BufferData(
        self.r#type,
        (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
        &data[0] as *const f32 as *const c_void,
        self.usage,
      )
    }
  }

  pub fn store_i32_data(&self, data: &[i32]) {
    unsafe {
      gl::BufferData(
        self.r#type,
        (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
        &data[0] as *const i32 as *const c_void,
        self.usage,
      )
    }
  }
}

pub struct VertexAttribute {
  pub index: gl::types::GLuint,
}

impl VertexAttribute {
  pub fn new(
    index: u32,
    size: i32,
    r#type: gl::types::GLenum,
    normalized: gl::types::GLboolean,
    stride: gl::types::GLsizei,
    pointer: *const c_void,
  ) -> VertexAttribute {
    unsafe {
      gl::VertexAttribPointer(index, size, r#type, normalized, stride, pointer);
    }

    VertexAttribute { index }
  }

  pub fn enable(&self) {
    unsafe {
      gl::EnableVertexAttribArray(self.index);
    }
  }

  pub fn disable(&self) {
    unsafe {
      gl::DisableVertexAttribArray(self.index);
    }
  }
}
