use gl;
use cgmath;

pub trait RenderDataPrimitive: 'static + Copy {
  fn to_i32_vec(vector: &[Self]) -> Vec<i32> {
    panic!("Not implemented for this type!");
  }

  fn to_u32_vec(vector: &[Self]) -> Vec<u32> {
    panic!("Not implemented for this type!");
  }

  fn to_f32_vec(vector: &[Self]) -> Vec<f32> {
    panic!("Not implemented for this type!");
  }
}

impl RenderDataPrimitive for i32 {
  fn to_i32_vec(vector: &[Self]) -> Vec<i32> {
    vector.to_vec()
  }
}

impl RenderDataPrimitive for u32 {
  fn to_u32_vec(vector: &[Self]) -> Vec<u32> {
    vector.to_vec()
  }
}

impl RenderDataPrimitive for f32 {
  fn to_f32_vec(vector: &[Self]) -> Vec<f32> {
    vector.to_vec()
  }
}

pub struct VertexBufferObject {
  id: gl::types::GLuint,
  usage: gl::types::GLenum,
}

impl VertexBufferObject {
  pub fn new(usage: gl::types::GLenum) -> VertexBufferObject {
    let mut id = 0;
    unsafe {
      gl::CreateBuffers(1, &mut id);
    }

    VertexBufferObject { id, usage }
  }

  pub fn generate(usage: gl::types::GLenum) -> VertexBufferObject {
    let mut id = 0;
    unsafe {
      gl::GenBuffers(1, &mut id);
    }

    VertexBufferObject { id, usage }
  }

  pub fn store(&self, data: &[gl::types::GLfloat]) {
    unsafe {
      gl::BufferData(
        gl::ARRAY_BUFFER,
        (data.len() * std::mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
        &data[0] as *const gl::types::GLfloat as *const std::ffi::c_void,
        self.usage
      );
    }
  }

  pub fn bind(&self) {
    unsafe {
      gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
    }
  }

  pub fn unbind() {
    unsafe {
      gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }
  }

  pub fn delete(self) {
    unsafe {
      gl::DeleteBuffers(1, &self.id);
    }
  }
}

pub struct ElementBufferObject {
  id: gl::types::GLuint,
  usage: gl::types::GLenum,
}

impl ElementBufferObject {
  pub fn new(usage: gl::types::GLenum) -> ElementBufferObject {
    let mut id = 0;
    unsafe {
      gl::CreateBuffers(1, &mut id);
    }

    ElementBufferObject { id, usage }
  }

  pub fn generate(usage: gl::types::GLenum) -> ElementBufferObject {
    let mut id = 0;
    unsafe {
      gl::GenBuffers(1, &mut id);
    }

    ElementBufferObject { id, usage }
  }

  pub fn store(&self, data: &[gl::types::GLuint]) {
    unsafe {
      gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        (data.len() * std::mem::size_of::<gl::types::GLuint>()) as gl::types::GLsizeiptr,
        &data[0] as *const gl::types::GLuint as *const std::ffi::c_void,
        self.usage
      );
    }
  }

  pub fn bind(&self) {
    unsafe {
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
    }
  }

  pub fn unbind() {
    unsafe {
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    }
  }

  pub fn delete(self) {
    unsafe {
      gl::DeleteBuffers(1, &self.id);
    }
  }
}

#[derive(Clone, Copy)]
pub enum Attribute {
  Float,
  Vec2,
  Vec3,
  Vec4,
  Int,
  IVec2,
  IVec3,
  IVec4
}

impl Attribute {
  fn size(&self) -> gl::types::GLsizei {
    match self {
      Attribute::Float | Attribute::Int => 1,
      Attribute::Vec2  | Attribute::IVec2 => 2,
      Attribute::Vec3  | Attribute::IVec3 => 3,
      Attribute::Vec4  | Attribute::IVec4 => 4,
    }
  }

  fn gl_type(&self) -> gl::types::GLenum {
    match self {
      Attribute::Float | Attribute::Vec2  | Attribute::Vec3  | Attribute::Vec4 => gl::FLOAT,
      Attribute::Int   | Attribute::IVec2 | Attribute::IVec3 | Attribute::IVec4 => gl::INT,
    }
  }

  fn normalized(&self) -> gl::types::GLboolean {
    match self {
      Attribute::Float | Attribute::Vec2 | Attribute::Vec3 | Attribute::Vec4 => gl::FALSE,
      _ => gl::FALSE, // Integers are not normalized
    }
  }
}

pub struct AttributeBuilder {
  attributes: Vec<Attribute>,
  stride: i32,
}

impl AttributeBuilder {
  pub fn new() -> AttributeBuilder {
    AttributeBuilder {
      attributes: Vec::new(),
      stride: 0,
    }
  }

  pub fn add(mut self, attribute: Attribute) -> Self {
    self.stride += attribute.size() * std::mem::size_of::<gl::types::GLfloat>() as i32;
    self.attributes.push(attribute);
    self
  }

  pub fn enable(&self) {
    let mut offset = 0;
    for (index, attribute) in self.attributes.iter().enumerate() {
      unsafe {
        gl::EnableVertexAttribArray(index as u32);
        gl::VertexAttribPointer(
          index as u32,
          attribute.size(),
          attribute.gl_type(),
          attribute.normalized(),
          self.stride,
          offset as *const std::ffi::c_void,
        );
      }

      offset += attribute.size() * std::mem::size_of::<gl::types::GLfloat>() as i32;
    }
  }
}

pub struct VertexDescriptor {
  pub attributes: Vec<Attribute>,
  pub stride: i32,
}

#[derive(Clone)]
pub struct VertexArrayObject(gl::types::GLuint);

impl VertexArrayObject {
  pub fn new() -> VertexArrayObject {
    let mut id: u32 = 0;
    unsafe {
      gl::CreateVertexArrays(1, &mut id);
    }

    VertexArrayObject(id)
  }

  pub fn from(
    usage: gl::types::GLenum,
    vertices: &[gl::types::GLfloat],
    indices: &[gl::types::GLuint],
    descriptor: VertexDescriptor,
  ) -> VertexArrayObject {
    let mut id: u32 = 0;
    let mut buffers: [u32; 2] = [ 0, 0 ];
    unsafe {
      gl::CreateVertexArrays(1, &mut id);

      gl::BindVertexArray(id);
      gl::CreateBuffers(2, &mut buffers[0]);
      gl::BindBuffer(gl::ARRAY_BUFFER, buffers[0]);
      gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertices.len() * std::mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
        &vertices[0] as *const f32 as *const std::ffi::c_void,
        usage
      );
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffers[1]);
      gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        (indices.len() * std::mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
        &indices[0] as *const u32 as *const std::ffi::c_void, 
        usage
      );
    }

    let stride: gl::types::GLsizei = descriptor.attributes.iter().map(|a| a.size()).sum::<i32>() * std::mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei;

    let mut offset = 0;
    for (index, attribute) in descriptor.attributes.iter().enumerate() {
      unsafe {
        gl::EnableVertexAttribArray(index as u32);
        gl::VertexAttribPointer(
          index as u32,
          attribute.size(),
          attribute.gl_type(),
          attribute.normalized(),
          stride,
          offset as *const std::ffi::c_void,
        );
      }
      offset += attribute.size() * std::mem::size_of::<gl::types::GLfloat>() as i32;
    }

    unsafe {
      gl::BindVertexArray(0);
      gl::BindBuffer(gl::ARRAY_BUFFER, 0);
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    }
    
    VertexArrayObject(id)
  }

  pub fn bind(&self) {
    unsafe {
      gl::BindVertexArray(self.0);
    }
  }

  pub fn unbind() {
    unsafe {
      gl::BindVertexArray(0);
    }
  }
}
