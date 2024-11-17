use cgmath::SquareMatrix;
use gl;
use std::mem;
use std::ffi::c_void;

use crate::graphics::shaders;
use crate::ecs::components::Texture2D;
use crate::debugger::debugger::*;

use super::shaders::ShaderProgram;

#[derive(Debug)]
pub struct VertexArrayObject {
  id: gl::types::GLuint,
}

impl VertexArrayObject {
  pub fn new() -> VertexArrayObject {
    let mut id: gl::types::GLuint = 0;
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

  pub fn unbind() {
    unsafe {
      gl::BindVertexArray(0);
    }
  }
}

#[derive(Debug)]
pub struct VertexBufferObject {
  id: gl::types::GLuint,     // id
  r#type: gl::types::GLenum, // target type
  usage: gl::types::GLenum,  // usage e.g draw call
}

impl VertexBufferObject {
  pub fn new(target: gl::types::GLenum, usage: gl::types::GLenum) -> VertexBufferObject {
    let mut id: gl::types::GLuint = 0;
    unsafe {
      gl::GenBuffers(1, &mut id);
    }

    VertexBufferObject { id, r#type: target, usage}
  }

  pub fn bind(&self) {
    unsafe { gl::BindBuffer(self.r#type, self.id); }
  }

  pub fn unbind(&self) {
    unsafe { gl::BindBuffer(self.r#type, 0); }
  }

  pub fn store_f32_data(&self, data: &[f32]) {
    unsafe {
      gl::BufferData(
        self.r#type,
        (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
        &data[0] as *const f32 as *const c_void,
        self.usage,
      );
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

#[derive(Debug)]
pub struct ElementBufferObject {
  size: isize,
  id: gl::types::GLuint,
  r#type: gl::types::GLenum,
  usage: gl::types::GLenum,
}

impl ElementBufferObject {
    pub fn new(r#type: gl::types::GLenum, usage: gl::types::GLenum) -> ElementBufferObject {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        ElementBufferObject { size: 0, id, r#type, usage }
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

    pub fn store_u32_data(&mut self, data: &[u32]) {
      self.size = (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr;
      unsafe {
          gl::BufferData(
              self.r#type,
              self.size,
              &data[0] as *const u32 as *const c_void,
              self.usage,
          )
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
    pointer: *const c_void      // The offset of where the position data begins in the buffer. If the position data is at the start of the data array this value is just 0
  ) -> VertexAttribute {
    unsafe {
      gl::VertexAttribPointer(location, size, type_of_data, normalized, stride, pointer);
    }

    VertexAttribute { location }
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
pub struct PhoenixRenderer {
  
}

/// Temporary game object for testing

#[derive(Debug)]
pub struct StaticGameObject {
  vertices: Vec<f32>,
  indices: Vec<u32>,
  texture: usize,
  vao: VertexArrayObject,
  vbo: VertexBufferObject,
  ebo: ElementBufferObject,
  vap: Vec<VertexAttribute>,
  shader_program: usize,
}

impl StaticGameObject {
  pub fn new(
    vertices: Vec<f32>,
    indices: Vec<u32>,
    texture: usize,
    shader_program: usize,
  ) -> StaticGameObject {
    let vao: VertexArrayObject = VertexArrayObject::new();
    vao.bind();

    let vbo: VertexBufferObject = VertexBufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();

    vbo.store_f32_data(vertices.as_slice());

    let mut ebo: ElementBufferObject = ElementBufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ebo.bind();

    ebo.store_u32_data(indices.as_slice());

    // Change these attributes later
    // They should maybe be choose-able
    let pos_v_attrib: VertexAttribute = VertexAttribute::new(
      0,
      3,
      gl::FLOAT,
      gl::FALSE,
      9 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei, // aPos (vec3) + aColor (vec4) + aTexCoord (vec2)
      0 as *const c_void,
    );

    let color_v_attrib: VertexAttribute = VertexAttribute::new(
      1,
      4,
      gl::FLOAT,
      gl::FALSE,
      9 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei, // aPos (vec3) + aColor (vec4) + aTexCoord (vec2)
      (3 * mem::size_of::<gl::types::GLfloat>()) as *const c_void,
    );

    let texture_v_attrib: VertexAttribute = VertexAttribute::new(
      2,
      2,
      gl::FLOAT,
      gl::FALSE,
      9 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei, // aPos (vec3) + aColor (vec4) + aTexCoord (vec2)
      (7 * mem::size_of::<gl::types::GLfloat>()) as *const c_void,
    );

    pos_v_attrib.enable();
    color_v_attrib.enable();
    texture_v_attrib.enable();

    StaticGameObject {
      vertices: vertices,
      indices: indices,
      vao: vao,
      vbo: vbo,
      ebo: ebo,
      vap: vec![pos_v_attrib, color_v_attrib, texture_v_attrib],
      shader_program: shader_program,
      texture: texture,
    }
  }

  pub fn render(&self, textures: &Vec<Texture2D>, shaders: &Vec<ShaderProgram>) {
    // bind texture
    textures.get(self.texture).unwrap().bind();

    // bind vao (vbo & ebo get included there)
    self.vao.bind();

    // bind its shader
    shaders.get(self.shader_program).unwrap().bind();
    
    // call/forward draw calls
    unsafe {
      gl::DrawElements(gl::TRIANGLES, self.ebo.size as i32, gl::UNSIGNED_INT, std::ptr::null()); // Last arg is an offset or index array (when not using indices)
      check_gl_error();
    }
  }
}

impl PhoenixRenderer {
  pub fn new() -> PhoenixRenderer {
    PhoenixRenderer {

    }
  }

  pub fn render(&mut self) /* -> PhoenixDebugInfo<()> */ {
    
  }
}

fn check_gl_error() {
  unsafe {
    let error = gl::GetError();
    if error != gl::NO_ERROR {
      println!("OpenGL error: {:?}", error);
    }
  }
}
