use glfw::Context;

use crate::{
  graphics::{
    mesh::Mesh,
    window
  },
  objects::{lights::PointLight, objects::GameObject}
};

pub struct PhoenixEngineInfo {}

impl PhoenixEngineInfo {
  pub fn get_vertex_attrib_count() -> i32 {
    let mut nr_attributes: i32 = 0;
    unsafe { gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut nr_attributes); }
    nr_attributes
  }

  pub fn get_texture_unit_count() -> i32 {
    let mut nr_attributes: i32 = 0;
    unsafe { gl::GetIntegerv(gl::MAX_TEXTURE_IMAGE_UNITS, &mut nr_attributes); }
    nr_attributes
  }
}

pub struct PhoenixApplication {
  window: window::Window,

  game_objects: Vec<GameObject>,
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
          game_objects: Vec::new(),
          point_lights: Vec::new(),
        }
      )
    )
  }

  pub fn add_static_object(&mut self, static_object: GameObject) {
    self.game_objects.push(static_object);
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
        
        for game_object in &self.game_objects {
          game_object.mesh.draw();
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
