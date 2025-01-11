use gl;
use glfw::{ *, Context};
use std::ffi::c_void;

use crate::vertices::*;
use crate::indices::*;
use crate::shaders::*;
use crate::scene;

pub struct Renderer {
  /*
  pub vaos: Vec<VertexArrayObject>,
  pub vbos: Vec<VertexBufferObject>,
  pub ibos: Vec<IndexBufferObject>, 
  pub vatrs: Vec<VertexAttribute>,
  pub sprogs: Vec<ShaderProgram>,
  */
}

impl Renderer {
  pub fn new() -> Renderer {
    Renderer {
      /*
      vaos: vec![],
      vbos: vec![],
      ibos: vec![],
      vatrs: vec![],
      sprogs: vec![],
      */
    }
  }

  pub fn render(&mut self, scene: &scene::Scene) {
    
  }
}
