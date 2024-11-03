use cgmath::SquareMatrix;
use gl;
use std::mem;
use std::ffi::c_void;

use crate::graphics::shaders;

use super::shaders::ShaderProgram;

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

pub struct ElementBufferObject {
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

        ElementBufferObject { id, r#type, usage }
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

pub struct PhoenixRenderer {
  
}

impl PhoenixRenderer {
  pub fn new() -> PhoenixRenderer {
    
    
    PhoenixRenderer {

    }
  }

  pub fn render(&mut self) /* -> PhoenixDebugInfo<()> */ {
    let data: [f32; 21] = [
    // Vertices                 Colors
       0.5, -0.5, 0.0,          1.0, 0.0, 0.0, 1.0,
      -0.5, -0.5, 0.0,          0.0, 1.0, 0.0, 1.0,
       0.0,  0.5, 0.0,          0.0, 0.0, 1.0, 1.0,
    ];

    let indices: [u32; 3] = [
      0, 1, 2,
    ];

    let vao: VertexArrayObject = VertexArrayObject::new();
    vao.bind();

    let vbo: VertexBufferObject = VertexBufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    
    vbo.store_f32_data(&data);

    let ebo: ElementBufferObject = ElementBufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ebo.bind();

    ebo.store_u32_data(&indices);

    let pos_v_attrib: VertexAttribute = VertexAttribute::new(
      0,
      3,
      gl::FLOAT,
      gl::FALSE,
      7 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei, // aPos (vec3) + aColor (vec4)
      0 as *const c_void,
    );

    pos_v_attrib.enable();

    let color_v_attrib: VertexAttribute = VertexAttribute::new(
      1,
      4,
      gl::FLOAT,
      gl::FALSE,
      7 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei, // aPos (vec3) + aColor (vec4)
      (3 * mem::size_of::<gl::types::GLfloat>()) as *const c_void,
    );

    color_v_attrib.enable();

    let mut shader_program: ShaderProgram = ShaderProgram::new("./shaders/default.vert", "./shaders/default.frag");
    shader_program.bind();

    unsafe {
      // Wireframe mode
      // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

      // Regular mode
      gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);

      gl::ClearColor(0.0, 0.0, 0.0, 0.4);
      check_gl_error();

      gl::Clear(gl::DEPTH_BUFFER_BIT);
      check_gl_error();
      
      gl::Clear(gl::COLOR_BUFFER_BIT);
      check_gl_error();

      vao.bind();

      gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null()); // Last arg is an offset or index array (when not using indices)
      check_gl_error();

      VertexArrayObject::unbind();
    }
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

pub struct RenderableMesh {
  // Contains OpenGL/GPU-specific data.
}
