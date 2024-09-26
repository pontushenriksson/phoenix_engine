use gl;
use glfw::{ *, Context};

use crate::vertices::*;
use crate::indices::*;
use crate::shaders::*;

pub struct Renderer {
  pub vaos: Vec<VertexArrayObject>,
  pub vbos: Vec<VertexBufferObject>,
  pub ibos: Vec<IndexBufferObject>, 
  pub vatrs: Vec<VertexAttribute>,
  pub sprogs: Vec<ShaderProgram>,
}

impl Renderer {
  pub fn new() -> Renderer {
    Renderer {
      vaos: vec![],
      vbos: vec![],
      ibos: vec![],
      vatrs: vec![],
      sprogs: vec![],
    }
  }

  pub fn render(&mut self, /* scene: &Scene */) {
    /* 
    unsafe {
      gl::ClearColor(0.7, 0.13, 0.17, 0.1);

      gl::Clear(gl::COLOR_BUFFER_BIT);

      gl::DrawElements(gl::TRIANGLES, 9, gl::UNSIGNED_INT, 0 as *const c_void);
    }
    */
  }
}
