use std::ptr;
use std::fs::File;
use std::io::Read;
use std::ffi::CString;
use std::collections::HashMap;
use gl;
use cgmath::Matrix;

pub struct ShaderProgram {
  pub program_handle: u32,
  pub uniform_ids: HashMap<String, gl::types::GLint>,
}

#[allow(temporary_cstring_as_ptr)]
impl ShaderProgram {
  pub fn new(vertex_path: &str, fragment_path: &str) -> ShaderProgram {
    let mut vertex_file: File = File::open(vertex_path)
      .unwrap_or_else(|err| panic!("Failed to open {}\n\terr| {}", vertex_path, err));
    let mut fragment_file: File = File::open(fragment_path)
      .unwrap_or_else(|err| panic!("Failed to open {}\n\terr| {}", fragment_path, err));

    let mut vertex_source: String = String::new();
    let mut fragment_source: String = String::new();

    vertex_file
      .read_to_string(&mut vertex_source)
      .expect("Failed to read vertex shader");
    fragment_file
      .read_to_string(&mut fragment_source)
      .expect("Failed to read fragment shader");

    unsafe {
      let vertex_shader: gl::types::GLuint = gl::CreateShader(gl::VERTEX_SHADER);
      let c_str_vert: CString = CString::new(vertex_source.as_bytes())
        .expect("Failed to convert Rust String content for Vertex Shader to c_str_vert");
      gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
      gl::CompileShader(vertex_shader);

      let fragment_shader: gl::types::GLuint = gl::CreateShader(gl::FRAGMENT_SHADER);
      let c_str_frag: CString = CString::new(fragment_source.as_bytes())
        .expect("Failed to convert Rust String content for Fragment Shader to c_str_frag");
      gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
      gl::CompileShader(fragment_shader);

      let program_handle: u32 = gl::CreateProgram();
      gl::AttachShader(program_handle, vertex_shader);
      gl::AttachShader(program_handle, fragment_shader);
      gl::LinkProgram(program_handle);
      gl::DeleteShader(vertex_shader);
      gl::DeleteShader(fragment_shader);

      ShaderProgram {
        program_handle,
        uniform_ids: HashMap::new(),
      }
    }
  }

  pub fn bind(&self) {
    unsafe {
      gl::UseProgram(self.program_handle);
    }
  }

  pub fn unbind() {
    unsafe {
      gl::UseProgram(0);
    }
  }

  pub fn create_uniform(&mut self, uniform_name: &str) {
    let uniform_location: i32 = unsafe {
      gl::GetUniformLocation(
        self.program_handle,
        CString::new(uniform_name).unwrap().as_ptr(),
      )
    };

    if uniform_location < 0 {
      panic!("Failed to locate uniform: {}", uniform_name);
    } else {
      self.uniform_ids.insert(uniform_name.to_string(), uniform_location);
    }
  }

  pub fn set_matric4fv_uniform(&self, uniform_name: &str, matrix: &cgmath::Matrix4<f32>) {
    unsafe {
      gl::UniformMatrix4fv(
        self.uniform_ids[uniform_name],
        1,
        gl::FALSE,
        matrix.as_ptr(),
      )
    }
  }
}
