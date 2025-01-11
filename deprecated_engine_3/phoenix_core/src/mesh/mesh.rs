use std::{ffi::c_void, mem};

use gl;
use async_trait::async_trait;
use crate::assets::loader::*;
use cgmath::Matrix;
use image::GenericImageView;

#[derive(Debug)]
pub struct VertexArrayObject {
  id: gl::types::GLuint,
}

impl VertexArrayObject {
  pub fn new() -> VertexArrayObject {
    let mut id: gl::types::GLuint = 0;
    unsafe {
      gl::CreateVertexArrays(1, &mut id);
      check_gl_error("gl::CreateVertexArrays(1, &mut id)");
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
      check_gl_error("gl::GenBuffers(1, &mut id)");
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
      check_gl_error("gl::GenBuffers(1, &mut id)");
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

// Temporary game object for testing

/*

#[derive(Debug)]
pub struct StaticGameObject {
  vertices: Vec<f32>,
  indices: Vec<u32>,
  texture: i128,
  vao: VertexArrayObject,
  vbo: VertexBufferObject,
  ebo: ElementBufferObject,
  vap: Vec<VertexAttribute>,
  shader_program: usize,
  position: cgmath::Vector3<f32>,
  pub model_matrix: cgmath::Matrix4<f32>,
}

impl StaticGameObject {
  pub fn new(
    vertices: Vec<f32>,
    indices: Vec<u32>,
    texture: i128,
    shader_program: usize,
    position: cgmath::Vector3<f32>,
    model_matrix: cgmath::Matrix4<f32>,
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
      12 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei, // aPos (vec3) + aColor (vec4) + aTexCoord (vec2) + aNormal (vec3) = 12
      0 as *const c_void,                                             // aPos is first so no offset (0)
    );

    let color_v_attrib: VertexAttribute = VertexAttribute::new(
      1,
      4,
      gl::FLOAT,
      gl::FALSE,
      12 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei, // aPos (vec3) + aColor (vec4) + aTexCoord (vec2) + aNormal (vec3)
      (3 * mem::size_of::<gl::types::GLfloat>()) as *const c_void,    // aColor is second so it has an offset equal to the length of all previous attibutes (vec3 = 3)
    );

    let texture_v_attrib: VertexAttribute = VertexAttribute::new(
      2,
      2,
      gl::FLOAT,
      gl::FALSE,
      12 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei, // aPos (vec3) + aColor (vec4) + aTexCoord (vec2) + aNormal (vec3)
      (7 * mem::size_of::<gl::types::GLfloat>()) as *const c_void,    // aTexCoord is third so it has an offset equal to the length of all previous attibutes (vec3 + vec4 = 7)
    );

    let normals_v_attrib: VertexAttribute = VertexAttribute::new(
      3,
      3,
      gl::FLOAT,
      gl::FALSE,
      12 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei, // aPos (vec3) + aColor (vec4) + aTexCoord (vec2) + aNormal (vec3)
      (9 * mem::size_of::<gl::types::GLfloat>()) as *const c_void,    // aTexCoord is last so it has an offset equal to the length of all previous attibutes (vec3 + vec4 + vec2 = 9)
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
      position: position,
      model_matrix: model_matrix * cgmath::Matrix4::from_translation(position), // Equivalent to glm::translate(model_matrix, position)
    }
  }

  pub fn render(&self, cameras: &mut Vec<Camera3D>, textures: &Vec<Texture2D>, shaders: &mut Vec<ShaderProgram>, window: &mut glfw::PWindow) {
    // bind texture if needed (To not select a texture, use a negative index such as -1)
    if self.texture > 0 {
      textures.get(self.texture as usize).unwrap().bind();
    }

    // bind vao (vbo & ebo get included there)
    self.vao.bind();

    // bind its shader
    shaders.get(self.shader_program).unwrap().bind();

    cameras.get_mut(0).unwrap().inputs(window);
    cameras.get_mut(0).unwrap().update_matrix(45.0, 0.1, 100.0);
    cameras.get_mut(0).unwrap().matrix(&mut shaders.get_mut(0).unwrap(), "camMatrix");

    
    // call/forward draw calls
    unsafe {
      gl::DrawElements(gl::TRIANGLES, self.ebo.size as i32, gl::UNSIGNED_INT, std::ptr::null()); // Last arg is an offset or index array (when not using indices)
      check_gl_error("gl::DrawElements(gl::TRIANGLES, self.ebo.size as i32, gl::UNSIGNED_INT, std::ptr::null())");
    }
  }

  // Maybe a bind and unbind function later?
}

pub struct Light {
  
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
    let view = cgmath::Matrix4::look_at_rh(self.position, self.position + self.orientation, self.up);
    let proj = cgmath::perspective(cgmath::Deg(fov), (self.window_width / self.window_height) as f32, near_plane, far_plane);

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

*/

// pub trait Texture { ... }

#[derive(Debug)]
pub enum TextureType {
  Diffuse,
  Specular,
}

#[derive(Debug)]
pub struct Texture {
  id: gl::types::GLuint,
  r#type: TextureType,
  // target: gl::types::GLenum,
}

impl Texture {
  pub fn new(file_path: &str, texture_type: TextureType) -> Texture {
    let img = image::open(file_path).expect("Failed to load texture");
    let (width, height) = img.dimensions();
    let img_data = img.to_rgba8(); // Convert to RGBA format

    // Flip the image vertically
    let row_size = (width * 4) as usize; // 4 bytes per pixel (RGBA)
    let mut flipped_data = vec![0u8; (width * height * 4) as usize];
    
    for y in 0..height {
      let src_offset = (y * width * 4) as usize;
      let dst_offset = ((height - 1 - y) * width * 4) as usize; // Flipping vertically

      // Correctly copy the row from img_data to flipped_data
      flipped_data[dst_offset..dst_offset + row_size]
        .copy_from_slice(&img_data.as_raw()[src_offset..src_offset + row_size]);
    }

    // Generate and bind a texture in OpenGL
    let mut texture_id: gl::types::GLuint = 0;
    unsafe {
      gl::GenTextures(1, &mut texture_id);
      gl::BindTexture(gl::TEXTURE_2D, texture_id);

      // Set texture parameters
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

      // Upload flipped texture data to OpenGL
      gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGBA as i32,
        width as i32,
        height as i32,
        0,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        flipped_data.as_ptr() as *const std::ffi::c_void,
      );
    }

    Texture { id: texture_id, r#type: texture_type }
  }

  pub fn new_from_gltf(data: &[u8], texture_type: TextureType) -> Texture {
    // Decode image data using the `image` crate
    let img = image::load_from_memory(data)
      .expect("Failed to decode texture data from glTF");
    let (width, height) = img.dimensions();
    let img_data = img.to_rgba8(); // Convert to RGBA format

    // Flip the image vertically for OpenGL
    let row_size = (width * 4) as usize;
    let mut flipped_data = vec![0u8; (width * height * 4) as usize];
    for y in 0..height {
      let src_offset = (y * width * 4) as usize;
      let dst_offset = ((height - 1 - y) * width * 4) as usize;
      flipped_data[dst_offset..dst_offset + row_size]
        .copy_from_slice(&img_data.as_raw()[src_offset..src_offset + row_size]);
    }

    // Create OpenGL texture
    let mut texture_id: gl::types::GLuint = 0;
    unsafe {
      gl::GenTextures(1, &mut texture_id);
      gl::BindTexture(gl::TEXTURE_2D, texture_id);

      // Set texture parameters
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

      // Upload texture data to OpenGL
      gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGBA as i32,
        width as i32,
        height as i32,
        0,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        flipped_data.as_ptr() as *const std::ffi::c_void,
      );

      // Generate mipmaps
      gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    Texture {
      id: texture_id,
      r#type: texture_type,
    }
  }

  pub fn bind(&self, texture_unit: u32) {
    unsafe {
      gl::ActiveTexture(gl::TEXTURE0 + texture_unit);
      gl::BindTexture(gl::TEXTURE_2D, self.id);
    }
  }

  pub fn unbind(&self) {
    unsafe {
      gl::BindTexture(gl::TEXTURE_2D, 0);
    }
  }

  pub fn to_mipmap(&self) {
    unsafe {
      gl::BindTexture(gl::TEXTURE_2D, self.id);
      gl::GenerateMipmap(gl::TEXTURE_2D); // Generate mipmaps
    }
  }
}

impl Drop for Texture {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteTextures(1, &self.id);
    }
  }
}

pub struct Transform {
  position: cgmath::Vector3<f32>,
  rotation: cgmath::Quaternion<f32>,
  scale: cgmath::Vector3<f32>,
}

pub struct Joint {
  name: String,
  parent: Option<usize>, // Parent joint index
  transform: cgmath::Matrix4<f32>
}

pub struct Skin {
  joints: Vec<Joint>, // Joints/Bones influencing the mesh
  inverse_bind_matrices: Vec<cgmath::Matrix4<f32>> // Used for skinning
}

pub struct Keyframe {
  time: f32,
  joint_transforms: Vec<Transform>
}

pub struct Animation {
  name: String,
  keyframes: Vec<Keyframe>,
}

pub struct Rig {
  joints: Vec<Joint>,
  animations: Vec<Animation>
}

pub struct RenderData {
  vao: VertexArrayObject,
  vbo: VertexBufferObject,
  ebo: ElementBufferObject
}

pub struct Mesh {
  render_data: RenderData,
  vertex_data: RawVertexData,
  indices: Vec<u32>,

  // textures ...
  pub diffuse_maps: Option<Vec<Texture>>,    // Multiple diffuse textures
  /*
  specular_maps: Option<Vec<Texture>>,   // Multiple specular textures
  normal_maps: Option<Vec<Texture>>,     // Multiple normal maps
  roughness_map: Option<Texture>,        // Single roughness map
  metalness_map: Option<Texture>,        // Single metallic map

  skin: Option<Skin>,
  */
}

impl Mesh {
  pub fn new(
    vertex_data: RawVertexData,
    indices: Vec<u32>,
    target: gl::types::GLenum,
    usage: gl::types::GLenum,
    ebo_type: gl::types::GLenum,
    diffuse_maps: Option<Vec<Texture>>
  ) -> Mesh {
    
    let vao: VertexArrayObject = VertexArrayObject::new();
    vao.bind();

    let vbo: VertexBufferObject = VertexBufferObject::new(target, usage); // E.g gl::ARRAY_BUFFER & gl::STATIC_DRAW
    vbo.bind();

    vbo.store_f32_data(vertex_data.data.as_slice());
    
    let mut ebo: ElementBufferObject = ElementBufferObject::new(ebo_type, usage); // E.g gl::ELEMENT_ARRAY_BUFFER
    ebo.bind();

    ebo.store_u32_data(indices.as_slice());

    unsafe {
      // Set up attribute pointers
      let stride = (vertex_data.stride * std::mem::size_of::<f32>()) as i32;
      gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, 0 as *const _); // Position
      gl::EnableVertexAttribArray(0);
      gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * std::mem::size_of::<f32>()) as *const _); // Normal
      gl::EnableVertexAttribArray(1);
      gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (6 * std::mem::size_of::<f32>()) as *const _); // Tex coords
      gl::EnableVertexAttribArray(2);

      gl::BindVertexArray(0);
    }

    Mesh {
      render_data: RenderData { vao, vbo, ebo },
      vertex_data,
      indices,
      diffuse_maps: diffuse_maps
      // ...
    }
  }

  pub fn draw(&self) {
    unsafe {
      self.render_data.vao.bind();
      gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, std::ptr::null());
      VertexArrayObject::unbind(); // gl::BindVertexArray(0);
    }
  }
}

#[async_trait]
impl Asset for Mesh {
  async fn load(
    path: &str,
    target: gl::types::GLenum,
    usage: gl::types::GLenum,
    ebo_type: gl::types::GLenum
  ) -> Mesh {
    load_gltf(path, target, usage, ebo_type).await
  }
}

fn check_gl_error(place: &str) {
  unsafe {
    let error = gl::GetError();
    if error != gl::NO_ERROR {
      println!("OpenGL error at {} : {:?}", place, error);
    }
  }
}
