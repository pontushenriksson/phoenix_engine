use core::PhoenixEngine;
use glfw::WindowEvent;
use tokio;

pub mod core;

pub mod assets {
  pub mod loader;
}

pub mod mesh {
  pub mod mesh;
}

pub mod events {
  pub mod events;
}

pub mod shaders {
  pub mod shaders;
}

pub mod ecs {
  pub mod ecs;
}

pub struct PhoenixApplication {
  engine: Box<PhoenixEngine>,
}

impl PhoenixApplication {
  pub fn new(
    window_width: u32,
    window_height: u32,
    title: &str,
    icon_path: &str
  ) -> Box<PhoenixApplication> {
    Box::new(PhoenixApplication {
      engine: PhoenixEngine::new(
        window_width,
        window_height,
        title,
        icon_path  
      )
    })
  }

  pub fn bind_input<F: Fn()>(&mut self, input: u32 /* input: WindowEvent */, func: F) {
    func();
    panic!("Bound function!");
  }

  pub fn run(&mut self) {
    self.engine.run();
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::{Arc, Mutex};
  use assets::loader::AssetLoader;
  use cgmath::Rotation3;
use glfw::Context;
  use mesh::mesh::*;
  use gl;
  use shaders::shaders::ShaderProgram;
  use ecs::ecs::*;

  #[tokio::test]
  async fn test() {
    let glfw = Arc::new(Mutex::new(glfw::init(glfw::fail_on_errors).unwrap()));

    glfw.lock().unwrap().window_hint(glfw::WindowHint::Resizable(true));
    glfw.lock().unwrap().window_hint(glfw::WindowHint::TransparentFramebuffer(true));

    let (mut window, receiver) = glfw
      .lock().unwrap()
      .create_window(800, 800, "Test Game lib.rs", glfw::WindowMode::Windowed)
      .expect("Failed to create GLFW window!");
    
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    unsafe {
      gl::Enable(gl::BLEND);
      gl::Enable(gl::DEPTH_TEST);
      gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    let mut ecs = ECSManager::new();

    ecs.register_component::<Position>();
    ecs.register_component::<Rotation>();
    ecs.register_component::<Velocity>();

    let entity = ecs.create_entity();

    ecs.add_component(entity, Position(cgmath::point3(0.0, 0.0, 0.0)));
    ecs.add_component(entity, Rotation(cgmath::Quaternion::from_angle_x()));
    ecs.add_component(entity, Velocity(1.0));

    let mut movement_system: MovementSystem;
    ecs.add_system(movement_system);

    let delta_time = 0.016;

    while !window.should_close() {
      glfw.lock().unwrap().poll_events();
      // self.update();

      unsafe {
        // Wireframe mode
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        // Regular mode
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);

        gl::ClearColor(0.0, 0.0, 0.0, 0.4);
        // check_gl_error();

        gl::Clear(gl::DEPTH_BUFFER_BIT);
        // check_gl_error();
        
        gl::Clear(gl::COLOR_BUFFER_BIT);
        // check_gl_error();

      }

      
      ecs.run_systems(delta_time);

      window.swap_buffers();
    }
  }
}
