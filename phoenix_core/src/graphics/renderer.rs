use cgmath::*;
use cgmath::Matrix4;
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

  // Maybe a bind and unbind function later?
}

#[derive(Debug)]
pub struct Camera3D { // Implement Perspective and Orthographic separetelly later
  pub matrix: cgmath::Matrix4<f32>,
  pub position: cgmath::Point3<f32>,
  pub orientation: cgmath::Vector3<f32>,
  pub up: cgmath::Vector3<f32>,
  pub window_width: i32,
  pub window_height: i32,
  pub speed: f32, // 0.1
  pub sensitivity: f32, // 100.0
  pub first_click: bool,
}

impl Camera3D {
  pub fn new(width: i32, height: i32, position: cgmath::Point3<f32>) -> Camera3D {
    Camera3D {
      matrix: cgmath::Matrix4::identity(),
      position: position,
      orientation: cgmath::vec3(0.0, 0.0, -1.0),
      up: cgmath::vec3(0.0, 1.0, 0.0),
      window_width: width,
      window_height: height,
      speed: 0.01,
      sensitivity: 100.0,
      first_click: true,
    }
  } // new::perspective()

  pub fn update_matrix(&mut self, fov: f32, near_plane: f32, far_plane: f32) {
    let mut view: cgmath::Matrix4<f32> = cgmath::Matrix4::identity();
    let mut proj: cgmath::Matrix4<f32> = cgmath::Matrix4::identity();
    view = cgmath::Matrix4::look_at_rh(self.position, self.position + self.orientation, self.up);
    proj = cgmath::perspective(cgmath::Deg(fov), (self.window_width / self.window_height) as f32, near_plane, far_plane);

    self.matrix = proj * view;
  }

  pub fn matrix(&self, shader_program: &mut ShaderProgram, uniform_name: &str /* String later maybe*/) {
    shader_program.create_uniform(uniform_name);
    shader_program.set_uniform_matrix_4_f32_vec(uniform_name, &self.matrix);
  }

  pub fn inputs(&mut self, window: &mut glfw::Window) {
    // Keyboard input

    if window.get_key(glfw::Key::W) == glfw::Action::Press {
      self.position += self.speed * self.orientation;
    }
    if window.get_key(glfw::Key::A) == glfw::Action::Press {
      self.position += self.speed * -cgmath::Vector3::normalize(cgmath::Vector3::cross(self.orientation, self.up));
    }
    if window.get_key(glfw::Key::S) == glfw::Action::Press {
      self.position += self.speed * -self.orientation;
    }
    if window.get_key(glfw::Key::D) == glfw::Action::Press {
      self.position += self.speed * cgmath::Vector3::normalize(cgmath::Vector3::cross(self.orientation, self.up));
    }
    if window.get_key(glfw::Key::Space) == glfw::Action::Press {
      self.position += self.speed * self.up;
    }
    if window.get_key(glfw::Key::LeftControl) == glfw::Action::Press {
      self.position += self.speed * -self.up;
    }
    if window.get_key(glfw::Key::LeftShift) == glfw::Action::Press {
      self.speed = 0.04;
    }
    if window.get_key(glfw::Key::D) == glfw::Action::Release {
      self.speed = 0.01;
    }
    if window.get_key(glfw::Key::Backspace) == glfw::Action::Press {
      self.position = cgmath::point3(0.0, 0.0, 2.0);
      self.orientation = cgmath::vec3(0.0, 0.0, -1.0);
    }

    // Mouse input

    if window.get_mouse_button(glfw::MouseButton::Button1) == glfw::Action::Press {
      window.set_cursor_mode(glfw::CursorMode::Hidden);

      if self.first_click {
          window.set_cursor_pos((self.window_width / 2) as f64, (self.window_height / 2) as f64);
          self.first_click = false;
      }

      let (mouse_x, mouse_y): (f64, f64) = window.get_cursor_pos();

      let rot_x: f32 = self.sensitivity * ((mouse_y - (self.window_height / 2) as f64) as f32) / self.window_height as f32;
      let rot_y: f32 = self.sensitivity * ((mouse_x - (self.window_width / 2) as f64) as f32) / self.window_width as f32;

      // Calculate new orientation for vertical rotation (pitch)
      let right: Vector3<f32> = self.orientation.cross(self.up).normalize();
      let pitch_quat = Quaternion::from_axis_angle(right, Deg(-rot_x));

      let new_orientation = pitch_quat * self.orientation;

      // Make sure the new orientation doesn't exceed the vertical limit
      let up_dot = new_orientation.dot(self.up);
      if up_dot.abs() < 0.99 {
        self.orientation = new_orientation;
      }

      // Apply horizontal rotation (yaw)
      let yaw_quat = Quaternion::from_axis_angle(self.up, Deg(-rot_y));
      self.orientation = yaw_quat * self.orientation;

      // Reset mouse position to the center of the screen
      window.set_cursor_pos((self.window_width / 2) as f64, (self.window_height / 2) as f64);
    }

    if window.get_mouse_button(glfw::MouseButton::Button1) == glfw::Action::Release {
      window.set_cursor_mode(glfw::CursorMode::Normal);
      self.first_click = true;
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
