use gl::types::*;
use cgmath::Matrix;
use std::{collections::HashMap, ffi::CString, fs::File, io::Read, ptr};

pub struct ShaderProgram {
  id: GLuint,
  uniform_indexes: HashMap<String, GLint>,
}

#[allow(temporary_cstring_as_ptr)]
impl ShaderProgram {
  pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> ShaderProgram {
    let mut vertex_shader_file = File::open(vertex_shader_path)
      .unwrap_or_else(|e| panic!("Failed to open {}\n\terr| {}", vertex_shader_path, e));
    let mut fragment_shader_file = File::open(fragment_shader_path)
      .unwrap_or_else(|e| panic!("Failed to open {}\n\terr| {}", vertex_shader_path, e));

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
      let c_str_vert = CString::new(vertex_shader_source.as_bytes())
          .expect("Failed to convert Rust String content for Vertex Shader to c_str_vert");
      gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
      gl::CompileShader(vertex_shader);

      // Connect with Debugger later for a cleaner solution

      let mut shader_result: gl::types::GLint = 0;

      gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut shader_result);

      if shader_result == 0 {
        let mut info_log: [i8; 512] = [0; 512];
        gl::GetShaderInfoLog(vertex_shader, 512, ptr::null_mut(), info_log.as_mut_ptr());
        println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED: \n\t {:?}\n", info_log);
      }

      // ...

      let fragment_shader: gl::types::GLuint = gl::CreateShader(gl::FRAGMENT_SHADER);
      let c_str_vert = CString::new(fragment_shader_source.as_bytes())
          .expect("Failed to convert Rust String content for Fragment Shader to c_str_vert");
      gl::ShaderSource(fragment_shader, 1, &c_str_vert.as_ptr(), ptr::null());
      gl::CompileShader(fragment_shader);

      // Connect with Debugger later for a cleaner solution

      gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut shader_result);

      if shader_result == 0 {
        let mut info_log: [i8; 512] = [0; 512];
        gl::GetShaderInfoLog(vertex_shader, 512, ptr::null_mut(), info_log.as_mut_ptr());
        println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED: \n\t {:?}\n", info_log);
      }

      // ...

      let program_handle = gl::CreateProgram();
      gl::AttachShader(program_handle, vertex_shader);
      gl::AttachShader(program_handle, fragment_shader);
      gl::LinkProgram(program_handle);

      gl::GetProgramiv(program_handle, gl::LINK_STATUS, &mut shader_result);

      if shader_result == 0 {
        let mut info_log: [i8; 512] = [0; 512];
        gl::GetProgramInfoLog(program_handle, 512, ptr::null_mut(), info_log.as_mut_ptr());
        println!("ERROR::SHADER::PROGRAM::LINKING_FAILED: \n\t {:?}\n", info_log);
      }

      gl::DeleteShader(vertex_shader);
      gl::DeleteShader(fragment_shader);

      ShaderProgram {
        id: program_handle,
        uniform_indexes: HashMap::new(),
      }
    }
  }

  /// Activate/use shader program
  pub fn bind(&self) {
    unsafe {
      gl::UseProgram(self.id);
    }
  }

  /// Deactivate shader program
  pub fn unbind() {
    unsafe {
      gl::UseProgram(0);
    }
  }

  pub fn create_uniform(&mut self, uniform_name: &str) {
    let uniform_location: i32 = unsafe {
      gl::GetUniformLocation(
        self.id,
        CString::new(uniform_name).unwrap().as_ptr(),
      )
    };

    if uniform_location < 0 {
      panic!("Failed to locate uniform: {} err: {}", uniform_name, uniform_location);
    } else {
      self.uniform_indexes
          .insert(uniform_name.to_string(), uniform_location);
    }
  }

  pub fn set_uniform_bool(
    &self,
    uniform_name: &str,
    value: bool
  ) {
    unsafe {
      gl::Uniform1i(self.uniform_indexes[uniform_name], value as i32);
    }
  }

  pub fn set_uniform_i32(
    &self,
    uniform_name: &str,
    value: i32
  ) {
    unsafe {
      gl::Uniform1i(self.uniform_indexes[uniform_name], value);
    }
  }

  pub fn set_uniform_f32(
    &self,
    uniform_name: &str,
    value: f32
  ) {
    unsafe {
      gl::Uniform1f(self.uniform_indexes[uniform_name], value);
    }
  }

  pub fn set_uniform_4_f32_vec(
    &self, uniform_name: &str,
    v0: gl::types::GLfloat,
    v1: gl::types::GLfloat, 
    v2: gl::types::GLfloat,
    v3: gl::types::GLfloat
  ) {
    unsafe {
      gl::Uniform4f(self.uniform_indexes[uniform_name], v0, v1, v2, v3);            
    }
  }

  pub fn set_uniform_matrix_4_f32_vec(
    &self, uniform_name: &str, 
    matrix: &cgmath::Matrix4<f32>
  ) {
    unsafe {
      gl::UniformMatrix4fv(
        self.uniform_indexes[uniform_name],
        1,
        gl::FALSE,
        matrix.as_ptr(),
      )
    }
  }
}
