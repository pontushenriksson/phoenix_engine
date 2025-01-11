use glfw::Context;

use crate::{
  graphics::{
    mesh::Mesh,
    window
  },
  objects::{lights::PointLight, objects::StaticObject}
};

pub struct PhoenixApplication {
  window: window::Window,

  static_objects: Vec<StaticObject>,
  point_lights: Vec<PointLight>,
}

impl PhoenixApplication {
  pub fn new(
    width: u32,
    height: u32,
    title: &str,
    icon: &str,
  ) -> Option<Box<PhoenixApplication>> {
    Option::Some(
      Box::new(
        PhoenixApplication {
          window: window::Window::new(
            width,
            height,
            title,
            icon,
          ),
          static_objects: Vec::new(),
          point_lights: Vec::new(),
        }
      )
    )
  }

  pub fn add_static_object(&mut self, static_object: StaticObject) {
    self.static_objects.push(static_object);
  }

  pub fn add_point_light(&mut self, point: PointLight) {
    self.point_lights.push(point);
  }

  pub fn run(&mut self) {
    while !self.window.window
      .lock()
      .unwrap()
      .should_close()
    {
      self.window.glfw.lock().unwrap().poll_events();
      unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        
        for static_object in &self.static_objects {
          static_object.mesh.draw();
        }

        for point_light in &self.point_lights {
          point_light.icon.mesh.draw();
        }
      }

      println!("Running!");
      
      self.window.window.lock().unwrap().swap_buffers();
    }
  }
}
