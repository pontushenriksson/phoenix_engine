use gl;
use cgmath;

pub trait DataPrimitive: 'static {}

impl DataPrimitive for i32 {}
impl DataPrimitive for u32 {}
impl DataPrimitive for f32 {}

pub struct RawVertexData<T> {
  pub data: Vec<T>,
  pub stride: usize,
}

impl<T: DataPrimitive> RawVertexData<T> {
  pub fn wrap(data: Vec<T>, stride: usize) -> RawVertexData<T> {
    assert!(stride > 0, "Stride must be greater than 0.");
    assert!(data.len() % stride == 0, "Data length must be a multiple of stride.");
    RawVertexData {
      data,
      stride,
    }
  }

  /*

  /// Extract positions (first 3 floats per vertex)
  pub fn positions(&self) -> Vec<[T; 3]> {
    self.data.chunks(self.stride).map(|chunk| {
      [chunk[0], chunk[1], chunk[2]] // Assuming positions are the first 3 floats
    }).collect()
  }

  /// Extract normals (next 3 floats per vertex after positions)
  pub fn normals(&self) -> Vec<[T; 3]> {
    self.data.chunks(self.stride).map(|chunk| {
      [chunk[3], chunk[4], chunk[5]] // Assuming normals follow positions
    }).collect()
  }

  /// Extract texture coordinates (next 2 floats after normals)
  pub fn tex_coords(&self) -> Vec<[T; 2]> {
    self.data.chunks(self.stride).map(|chunk| {
      [chunk[6], chunk[7]] // Assuming tex coords follow normals
    }).collect()
  }

  */
}

pub struct RawIndexData<T> {
  pub data: Vec<T>,
}

impl<T: DataPrimitive> RawIndexData<T> {
  pub fn wrap(data: Vec<T>) -> RawIndexData<T> {
    RawIndexData {
      data,
    }
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

#[derive(Clone)]
pub struct VertexArrayObject(gl::types::GLuint);

impl VertexArrayObject {
  pub fn from(
    usage: gl::types::GLenum,
    vertices: &[gl::types::GLfloat],
    indices: &[gl::types::GLuint],
    attributes: &[Attribute],
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

    let stride: gl::types::GLsizei = attributes.iter().map(|a| a.size()).sum::<i32>() * std::mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei;

    let mut offset = 0;
    for (index, attribute) in attributes.iter().enumerate() {
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
