use std::sync::{Arc, Mutex};

use gl;
use glfw::PWindow;
use crate::core::PhoenixApplication;
use crate::gl_call;
use crate::debugger::debugger::Debugger;
use crate::graphics::window::Window;

pub trait Render {
  fn set_model_matrix(&mut self);
  fn set_view_matrix(&mut self, view: cgmath::Matrix4<f32>);
  fn set_projection_matrix(&mut self, projection: cgmath::Matrix4<f32>);
}

pub trait DebugRender {
  fn set_model_matrix(&mut self);
  fn set_view_matrix(&mut self, view: cgmath::Matrix4<f32>);
  fn set_projection_matrix(&mut self, projection: cgmath::Matrix4::<f32>);
}

pub struct Renderer {}

impl Renderer {
  pub fn clear() {
    unsafe {
      gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT));
    }
  }

  pub fn depth_test(set: bool) {
    unsafe {
      if set {
        gl_call!(gl::Enable(gl::DEPTH_TEST));
      } else {
        gl_call!(gl::Disable(gl::DEPTH_TEST));
      }
    }
  }

  pub fn blend(set: bool) {
    unsafe {
      if set {
        gl_call!(gl::Enable(gl::BLEND));
      } else {
        gl_call!(gl::Disable(gl::BLEND));
      }
    }
  }

  pub fn blend_function(sfactor: gl::types::GLenum, dfactor: gl::types::GLenum) {
    unsafe {
      gl_call!(gl::BlendFunc(sfactor, dfactor));
    }
  }

  pub fn cull_face(set: bool) {
    unsafe {
      if set {
        gl_call!(gl::Enable(gl::CULL_FACE));
      } else {
        gl_call!(gl::Disable(gl::CULL_FACE));
      }
    }
  }

  pub fn regular_mode() {
    unsafe {
      gl_call!(gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL));
    }
  }

  pub fn debug_mode() {
    unsafe {
      gl_call!(gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE));
    }
  }
}
