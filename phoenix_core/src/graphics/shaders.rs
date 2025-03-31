use cgmath::{self, Matrix};
use std::io::Read;

use crate::gl_call;
use crate::debugger::debugger::Debugger;

#[derive(Clone)]
pub struct ShaderProgram {
  id: gl::types::GLuint,
  pub uniforms: std::collections::HashMap<String, gl::types::GLint>,
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
      let vertex_shader: gl::types::GLuint = gl_call!(gl::CreateShader(gl::VERTEX_SHADER));
      let c_str_vert = std::ffi::CString::new(vertex_shader_source.as_bytes())
        .expect("Failed to convert Rust String content for Vertex Shader to c_str_vert");
      gl_call!(gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), std::ptr::null()));
      gl_call!(gl::CompileShader(vertex_shader));

      let mut success = gl::FALSE as gl::types::GLint;
      let mut info_log = vec![0; 512];

      // Check vertex shader compilation
      gl_call!(gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success));
      if success == gl::FALSE as gl::types::GLint {
        gl::GetShaderInfoLog(vertex_shader, 512, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut gl::types::GLchar);
        panic!("Vertex Shader Compilation Failed: {:?}", std::ffi::CStr::from_ptr(info_log.as_ptr()));
      }

      let fragment_shader: gl::types::GLuint = gl::CreateShader(gl::FRAGMENT_SHADER);
      let c_str_frag = std::ffi::CString::new(fragment_shader_source.as_bytes())
        .expect("Failed to convert Rust String content for Fragment Shader to c_str_vert");
      gl_call!(gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), std::ptr::null()));
      gl_call!(gl::CompileShader(fragment_shader));

      // Check fragment shader compilation
      gl_call!(gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success));
      if success == gl::FALSE as gl::types::GLint {
        gl::GetShaderInfoLog(fragment_shader, 512, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut gl::types::GLchar);
        panic!("Fragment Shader Compilation Failed: {:?}", std::ffi::CStr::from_ptr(info_log.as_ptr()));
      }

      let id = gl_call!(gl::CreateProgram());
      gl_call!(gl::AttachShader(id, vertex_shader));
      gl_call!(gl::AttachShader(id, fragment_shader));
      gl_call!(gl::LinkProgram(id));

      
      // Check program linking
      gl_call!(gl::GetProgramiv(id, gl::LINK_STATUS, &mut success));
      if success == gl::FALSE as gl::types::GLint {
        gl::GetProgramInfoLog(id, 512, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut gl::types::GLchar);
        panic!("Shader Program Linking Failed: {:?}", std::ffi::CStr::from_ptr(info_log.as_ptr()));
      }

      gl_call!(gl::DeleteShader(vertex_shader));
      gl_call!(gl::DeleteShader(fragment_shader));

      let mut uniforms = std::collections::HashMap::new();

      // uModel

      let mut location = gl_call!(gl::GetUniformLocation(
        id,
        std::ffi::CString::new("uModel").unwrap().as_ptr()
      ));
  
      if location < 0 {
        panic!("Failed to locate a uniform 'mat4' in shader constructed from:\n        ~\"{}\"\n      & ~\"{}\"", vertex, fragment);
      }
  
      uniforms.insert("uModel".to_string(), location);

      // uView

      location = gl_call!(gl::GetUniformLocation(
        id,
        std::ffi::CString::new("uView").unwrap().as_ptr()
      ));
  
      if location < 0 {
        panic!("Failed to locate a uniform 'mat4' in shader constructed from:\n        ~\"{}\"\n      & ~\"{}\"", vertex, fragment);
      }

      uniforms.insert("uView".to_string(), location);

      // uProjection

      location = gl_call!(gl::GetUniformLocation(
        id,
        std::ffi::CString::new("uProjection").unwrap().as_ptr()
      ));
  
      if location < 0 {
        panic!("Failed to locate a uniform 'mat4' in shader constructed from:\n        ~\"{}\"\n      & ~\"{}\"", vertex, fragment);
      }

      uniforms.insert("uProjection".to_string(), location);

      ShaderProgram {
        id,
        uniforms,
      }
    }
  }

  pub fn activate(&self) {
    unsafe {
      gl_call!(gl::UseProgram(self.id));
    }
  }

  pub fn deactivate() {
    unsafe {
      gl_call!(gl::UseProgram(0));
    }
  }

  pub fn create_uniform(&mut self, uniform: &str) {
    let location = unsafe {
      gl_call!(gl::GetUniformLocation(
        self.id,
        std::ffi::CString::new(uniform).unwrap().as_ptr()
      ))
    };

    if location < 0 {
      panic!("Failed to locate uniform: {}", uniform);
    }

    self.uniforms.insert(uniform.to_string(), location);
  }

  pub fn set_uniform_float(&self, uniform: &str, float: gl::types::GLfloat) {
    let msg = format!("No entry found for key: {}", uniform);
    unsafe {
      gl_call!(gl::Uniform1f(
        *self.uniforms.get(uniform).expect(&msg),
        float
      ));
    }
  }

  pub fn set_matrix4_f32(&self, uniform: &str, matrix: &cgmath::Matrix4<f32>) {
    let msg = format!("No entry found for key: {}", uniform);
    unsafe {
      gl_call!(gl::UniformMatrix4fv(
        *self.uniforms.get(uniform).expect(&msg),
        1,
        gl::FALSE,
        matrix.as_ptr()
      ));
    }
  }

  pub fn set_texture_unit(&self, uniform: &str, unit: i32) {
    let msg = format!("No entry found for key: {}", uniform);
    unsafe {
      gl_call!(gl::Uniform1i(
        *self.uniforms.get(uniform).expect(&msg),
        unit
      ));
    }
  }
}
