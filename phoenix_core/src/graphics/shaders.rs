use cgmath::{self, Matrix};
use std::io::Read;

pub struct ShaderProgram {
  id: gl::types::GLuint,
  uniforms: std::collections::HashMap<String, gl::types::GLint>,
}

#[allow(temporary_cstring_as_ptr)]
impl ShaderProgram {
  pub fn new(vertex: &str, fragment: &str) -> ShaderProgram {
    let mut vertex_shader_file = std::fs::File::open(vertex)
        .unwrap_or_else(|e| panic!("Failed to open {}\n\terr| {}", vertex, e));
    let mut fragment_shader_file = std::fs::File::open(fragment)
        .unwrap_or_else(|e| panic!("Failed to open {}\n\terr| {}", vertex, e));

    let mut vertex_shader_source = String::new();
    let mut fragment_shader_source = String::new();   
    
    vertex_shader_file
        .read_to_string(&mut vertex_shader_source)
        .expect("Failed to read vertex shader");
    fragment_shader_file
        .read_to_string(&mut fragment_shader_source)
        .expect("Failed to read fragment shader");

    unsafe {
      let vertex_shader: gl::types::GLuint = gl::CreateShader(gl::VERTEX_SHADER);
      let c_str_vert = std::ffi::CString::new(vertex_shader_source.as_bytes())
          .expect("Failed to convert Rust String content for Vertex Shader to c_str_vert");
      gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), std::ptr::null());
      gl::CompileShader(vertex_shader);

      let fragment_shader: gl::types::GLuint = gl::CreateShader(gl::FRAGMENT_SHADER);
      let c_str_frag = std::ffi::CString::new(fragment_shader_source.as_bytes())
          .expect("Failed to convert Rust String content for Fragment Shader to c_str_vert");
      gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), std::ptr::null());
      gl::CompileShader(fragment_shader);

      let id = gl::CreateProgram();
      gl::AttachShader(id, vertex_shader);
      gl::AttachShader(id, fragment_shader);
      gl::LinkProgram(id);
      gl::DeleteShader(vertex_shader);
      gl::DeleteShader(fragment_shader);

      ShaderProgram {
        id,
        uniforms: std::collections::HashMap::new(),
      }
    }
  }

  pub fn activate(&self) {
    unsafe {
      gl::UseProgram(self.id);
    }
  }

  pub fn deactivate() {
    unsafe {
      gl::UseProgram(0);
    }
  }

  pub fn create_uniform(&mut self, uniform: &str) {
    let location = unsafe {
      gl::GetUniformLocation(
        self.id,
        std::ffi::CString::new(uniform).unwrap().as_ptr()
      )
    };

    if location < 0 {
      panic!("Failed to locate uniform: {}", uniform);
    }

    self.uniforms.insert(uniform.to_string(), location);
  }

  pub fn set_matrix4_f32(&self, uniform: &str, matrix: &cgmath::Matrix4<f32>) {
    unsafe {
      gl::UniformMatrix4fv(
        self.uniforms[uniform],
        1,
        gl::FALSE,
        matrix.as_ptr()
      );
    }
  }

  pub fn set_texture_unit(&self, uniform: &str, unit: i32) {
    unsafe {
      gl::Uniform1i(
        self.uniforms[uniform],
        unit
      );
    }
  }
}
