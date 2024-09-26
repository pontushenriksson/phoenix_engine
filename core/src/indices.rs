use gl;
use std::mem;
use std::ffi::c_void;

pub struct IndexBufferObject {
  pub id: gl::types::GLuint,
  pub r#type: gl::types::GLenum,
  pub usage: gl::types::GLenum,
}

impl IndexBufferObject {
  pub fn new(r#type: gl::types::GLenum, usage: gl::types::GLenum) -> IndexBufferObject {
    let mut id: u32 = 0;
    unsafe {
      gl::CreateBuffers(1, &mut id);
    }

    IndexBufferObject { id, r#type, usage }
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

  pub fn store_u32_data(&self, data: &[u32]) {
    unsafe {
      gl::BufferData(
        self.r#type,
        (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
        &data[0] as *const u32 as *const c_void,
        self.usage,
      )
    }
  }
}
