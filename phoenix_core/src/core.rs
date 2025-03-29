use std::sync::{Arc, Mutex};

use cgmath::Rotation3;
use glfw::{Context, Glfw};
use crate::objects::geometry::Ground;
use crate::Cell;

use crate::{
  debugger::debugger::Debugger, gl_call, graphics::{
    camera::Camera, renderer::Renderer, window
  }, objects::{lights::PointLight, objects::GameObject, transform::Transform}
};

pub struct PhoenixEngineInfo {
  nr_attributes: i32,
  nr_texture_units: i32,
}

impl PhoenixEngineInfo {
  pub fn new() -> PhoenixEngineInfo {
    let mut nr_attributes: i32 = 0;
    unsafe { gl_call!(gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut nr_attributes)); }
    println!("Maximum available vertex attributes: {}", nr_attributes);
    
    let mut nr_texture_units: i32 = 0;
    unsafe { gl_call!(gl::GetIntegerv(gl::MAX_TEXTURE_IMAGE_UNITS, &mut nr_texture_units)); }
    println!("Maximum available texture units: {}", nr_texture_units);
    
    PhoenixEngineInfo {
      nr_attributes,
      nr_texture_units
    }
  }

  pub fn get_vertex_attrib_count() -> i32 {
    let mut nr_attributes: i32 = 0;
    unsafe { gl_call!(gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut nr_attributes)); }
    nr_attributes
  }

  pub fn get_texture_unit_count() -> i32 {
    let mut nr_texture_units: i32 = 0;
    unsafe { gl_call!(gl::GetIntegerv(gl::MAX_TEXTURE_IMAGE_UNITS, &mut nr_texture_units)); }
    nr_texture_units
  }

  pub fn vertex_attrib_count(&self) -> i32 {
    self.nr_attributes
  }

  pub fn texture_unit_count(&self) -> i32 {
    self.nr_texture_units
  }
}

struct Timer {
  last_frame: f64,
  delta_time: f64,
}

impl Timer {
  fn new(last_frame_time: f64) -> Timer {
    Timer {
      last_frame: last_frame_time,
      delta_time: 0.0,
    }
  }

  fn update(&mut self, glfw: &Arc<Mutex<Glfw>>) {
    let current_time = glfw.lock().unwrap().get_time();
    self.delta_time = current_time - self.last_frame;
    self.last_frame = current_time;
  }

  fn get_delta_time(&self) -> f64 {
    self.delta_time
  }
}

pub struct PhoenixApplication {
  pub window: window::Window,
  pub info: PhoenixEngineInfo,

  timer: Timer,
  pub game_objects: Vec<GameObject>,
  pub pointlights: Vec<PointLight>,
  pub grounds: Vec<Ground>,
  pub camera: Cell<Camera>,
}

impl PhoenixApplication {
  pub fn new(
    width: u32,
    height: u32,
    title: &str,
    icon: &str,
  ) -> Option<Box<PhoenixApplication>> {
    let window = window::Window::new(
      width,
      height,
      title,
      icon,
    );

    println!("------------------------------ Phoenix Application ------------------------------");

    let info = PhoenixEngineInfo::new();
    let last_frame = window.glfw.lock().unwrap().get_time();

    println!("------------------------------------- Game --------------------------------------");

    Option::Some(
      Box::new(
        PhoenixApplication {
          window,
          info: info,
          timer: Timer::new(last_frame),
          game_objects: Vec::new(),
          pointlights: Vec::new(),
          grounds: Vec::new(),
          camera: Cell::Dead,
        }
      )
    )
  }

  pub fn add_game_object(&mut self, object: GameObject) {
    self.game_objects.push(object);
    println!("[Runtime] Add game object");
  }

  pub fn add_pointlight(&mut self, point: PointLight) {
    self.pointlights.push(point);
    println!("[Runtime] Add pointlight");
  }

  pub fn add_ground(&mut self, ground: Ground) {
    self.grounds.push(ground);
    println!("[Runtime] Add ground");
  }

  pub fn add_camera(&mut self, camera: Camera) {
    self.camera = Cell::Alive(camera);
    println!("[Runtime] Add camera");
  }

  pub fn run(&mut self) {
    println!("--------------------------------- Running Game ----------------------------------");
    
    let mut delta_time: f64;
    let mut target_fps = 60.0; // Start with 60 FPS
    let mut frame_time = std::time::Duration::from_secs_f64(1.0 / target_fps);

    let mut theta: f32 = 0.0;

    while !self.window.window.lock().unwrap().should_close() {
      if theta >= 360.0 {
        theta = 0.0;
      }

      theta += 0.1;

      let now = std::time::Instant::now();

      // Handle pulled events
      self.window.event_manager.accumulate();
      self.window.event_manager.handle();

      // Update timing
      self.timer.update(&self.window.glfw);
      delta_time = self.timer.get_delta_time();

      // Render
      {
        Renderer::clear();

        let camera = self.camera.alive();
        camera.inputs(&self.window.window, delta_time);
        camera.update_matrix();

        for object in &mut self.game_objects {
          object.update_matrix();
          object.draw(&camera);
        }

        for point in &mut self.pointlights {
          point.update_matrix();
          point.draw(&camera);
        }

        unsafe {
          gl::Disable(gl::CULL_FACE);
        }

        for ground in &mut self.grounds {
          ground.set_rotation(cgmath::Quaternion::from_angle_y(cgmath::Deg(theta)));
          ground.update_matrix();
          ground.draw(&camera, delta_time as f32);
        }

        unsafe {
          gl::Enable(gl::CULL_FACE);
        }
      }

      // Swap buffers
      self.window.window.lock().unwrap().swap_buffers();

      // Measure frame time
      let elapsed = now.elapsed();

      // **Adaptive FPS Adjustment**
      if elapsed > frame_time {
        target_fps *= 0.95; // Reduce FPS if rendering takes too long
        target_fps = target_fps.max(30.0); // Prevent dropping too low
      } else {
        target_fps *= 1.05; // Increase FPS if rendering is too fast
        target_fps = target_fps.min(144.0); // Cap at 144 FPS
      }

      frame_time = std::time::Duration::from_secs_f64(1.0 / target_fps);

      // Sleep to maintain frame rate
      if elapsed < frame_time {
        std::thread::sleep(frame_time - elapsed);
      }

      print!("[Runtime] Running at {:.1} fps\r", target_fps);
    }
  }
}
