fn check_gl_error(place: &str) {
  unsafe {
    let error = gl::GetError();
    if error != gl::NO_ERROR {
      println!("OpenGL error at {} : {:?}", place, error);
    }
  }
}

#[derive(Debug)]
pub struct VertexAttributeDescriptor {
  pub location: gl::types::GLuint, // Shader Location
  pub size: gl::types::GLint, // Components per attribute (e.g 3 for vec3)
  pub data_type: gl::types::GLenum, // Data type (e.g gl::FLOAT)
  pub normalized: gl::types::GLboolean,
  pub stride: gl::types::GLsizei,
  pub offset: usize, // Offset within stride
}

#[derive(Debug)]
pub struct VertexBufferObject {
  id: gl::types::GLuint,     // id
  target: gl::types::GLenum, // target type, e.g gl::ARRAY_BUFFER
  usage: gl::types::GLenum,  // usage/draw call, e.g gl::STATIC_DRAW
}

impl VertexBufferObject {
  pub fn new(
    target: gl::types::GLenum,
    usage: gl::types::GLenum
  ) -> VertexBufferObject {
    let mut id: gl::types::GLuint = 0;
    unsafe {
      // Depends on OpenGL version either Gen or Create
      gl::CreateBuffers(1, &mut id);
      // TODO: Debug
      check_gl_error("gl::CreateBuffers(1, &mut id)");
    }

    VertexBufferObject { id, target, usage}
  }

  pub fn bind(&self) {
    unsafe {
      gl::BindBuffer(self.target, self.id);
    }
  }

  pub fn unbind(&self) {
    unsafe {
      gl::BindBuffer(self.target, 0);
    }
  }

  pub fn store_data<T>(&self, data: &[T]) {
    unsafe {
      // gl::BindBuffer(self.target, self.id);
      gl::BufferData(
        self.target,
        (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
        data.as_ptr() as *const std::ffi::c_void,
        self.usage,
      );
      // gl::BindBuffer(self.target, 0);
    }
  }
}

impl Drop for VertexBufferObject {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteBuffers(1, &self.id);
    }
  }
}

#[derive(Debug)]
pub struct VertexAttribute {
  location: gl::types::GLuint, // location in layout in shader
}

impl VertexAttribute {
  pub fn new(
    location: gl::types::GLuint,
    size: gl::types::GLint,
    type_of_data: gl::types::GLenum, // a vec* (vec type) in GLSL consists of floating point values so this should be gl::FLOAT
    normalized: gl::types::GLboolean,
    /*
    
    If we're inputting integer data types (int, byte) and we've set this to GL_TRUE,
    the integer data is normalized to 0 (or -1 for signed data) and 1 when converted to float. 
    This is not relevant for us so we'll leave this at gl::FALSE.
    
    */
    stride: gl::types::GLsizei,
    pointer: *const std::ffi::c_void      // The offset of where the position data begins in the buffer. If the position data is at the start of the data array this value is just 0
  ) -> VertexAttribute {
    unsafe {
      gl::EnableVertexAttribArray(location);
      gl::VertexAttribPointer(location, size, type_of_data, normalized, stride, pointer);
    }

    VertexAttribute { location }
  }

  pub fn from(descriptor: &VertexAttributeDescriptor) -> VertexAttribute {
    VertexAttribute::new(
      descriptor.location,
      descriptor.size,
      descriptor.data_type,
      descriptor.normalized,
      descriptor.stride,
      (descriptor.offset * std::mem::size_of::<f32>()) as *const std::ffi::c_void
    )
  }

  pub fn enable(&self) {
    unsafe {
      gl::EnableVertexAttribArray(self.location);
    }
  }

  pub fn disable(&self) {
    unsafe {
      gl::DisableVertexAttribArray(self.location);
    }
  }
}

#[derive(Debug)]
pub struct VertexArrayObject {
  id: gl::types::GLuint,
}

impl VertexArrayObject {
  pub fn new() -> VertexArrayObject {
    let mut id: gl::types::GLuint = 0;
    unsafe {
      // Depends on OpenGL version either Gen or Create
      gl::CreateVertexArrays(1, &mut id);
      // TODO: Debug
      check_gl_error("gl::CreateVertexArrays(1, &mut id)");
    }

    VertexArrayObject { id }
  }

  pub fn link_vbo(
    &self,
    vbo: &VertexBufferObject,
    attributes: &[VertexAttributeDescriptor],
    stride: gl::types::GLsizei
  ) {
    vbo.bind();
    for attribute in attributes {
      unsafe {
        gl::VertexAttribPointer(
          attribute.location,
          attribute.size,
          attribute.data_type,
          attribute.normalized,
          stride,
          attribute.offset as *const std::ffi::c_void
        );
      }
    }
    vbo.unbind();
  }

  pub fn link_separate_vbo(
    &self,
    vbo: &VertexBufferObject,
    descriptor: &VertexAttributeDescriptor
  ) {
    vbo.bind();
    unsafe {
      gl::VertexAttribPointer(
        descriptor.location,
        descriptor.size,
        descriptor.data_type,
        descriptor.normalized,
        0, // No stride since the data is non-interleaved
        descriptor.offset as *const std::ffi::c_void
      );
    }
    vbo.unbind();
  }

  pub fn bind(&self) {
    unsafe {
      gl::BindVertexArray(self.id);
    }
  }

  pub fn unbind() {
    unsafe {
      gl::BindVertexArray(0);
    }
  }
}

#[derive(Debug)]
pub struct ElementBufferObject {
  id: gl::types::GLuint,
  target: gl::types::GLenum,
  usage: gl::types::GLenum,
}

impl ElementBufferObject {
  pub fn new(target: gl::types::GLenum, usage: gl::types::GLenum) -> ElementBufferObject {
    let mut id = 0;
    unsafe {
      // Depends on OpenGL version either Gen or Create
      gl::CreateBuffers(1, &mut id);
      // TODO: Debug
      check_gl_error("gl::CreateBuffers(1, &mut id)");
    }

    ElementBufferObject { id, target, usage }
  }

  pub fn bind(&self) {
    unsafe {
      gl::BindBuffer(self.target, self.id);
    }
  }

  pub fn unbind(&self) {
    unsafe {
      gl::BindBuffer(self.target, 0);
    }
  }

  pub fn store_data(&mut self, data: &[gl::types::GLuint]) {
    unsafe {
      // gl::BindBuffer(self.target, self.id);
      gl::BufferData(
        self.target,
        (data.len() * std::mem::size_of::<gl::types::GLuint>()) as isize,
        &data[0] as *const u32 as *const std::ffi::c_void,
        self.usage,
      );
      // gl::BindBuffer(self.target, 0);
    }
  }
}

pub struct RenderData {
  pub vao: VertexArrayObject,
  pub vbo: VertexBufferObject,
  pub ebo: Option<ElementBufferObject>,
  // vapds: Vec<VertexAttributeDescriptor>,
  // vaps: Vec<VertexAttribute>,
}
