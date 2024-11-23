use gl;
use glfw::{Context, Glfw, GlfwReceiver, PWindow, WindowEvent};
use image::GenericImageView;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use crate::events::events::EventsManager;

pub trait Layer {
  fn on_attach(&mut self) {}
  fn on_update(&mut self) {}
}

pub struct PhoenixEngine {
  glfw: Arc<Mutex<Glfw>>,
  window: Arc<Mutex<PWindow>>,
  events_manager: EventsManager,
  layers: Vec<Box<dyn Layer>>,
}

impl PhoenixEngine {
  pub fn new(
    window_width: u32,
    window_height: u32,
    title: &str,
    icon_path: &str
  ) -> Box<PhoenixEngine> {
    let glfw = Arc::new(Mutex::new(glfw::init(glfw::fail_on_errors).unwrap()));

    glfw.lock().unwrap().window_hint(glfw::WindowHint::Resizable(true));
    glfw.lock().unwrap().window_hint(glfw::WindowHint::TransparentFramebuffer(true));

    let (mut window, receiver) = glfw
      .lock().unwrap()
      .create_window(window_width, window_height, title, glfw::WindowMode::Windowed)
      .expect("Failed to create GLFW window!");
    
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    let icon = match image::open(icon_path) {
      Ok(icon) => icon,
      Err(e) => { panic!("Failed to open path to icon: {} \n\terr| {}", icon_path, e) }
    };

    let (width, height) = icon.dimensions();

    let icon_pixels = rgba_u8_as_u32(icon.to_rgba8().into_raw());

    let glfw_icon = glfw::PixelImage {
      width: width as u32,
      height: height as u32,
      pixels: icon_pixels,
    };

    window.set_icon_from_pixels(vec![glfw_icon]);

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let window = Arc::new(Mutex::new(window));
    let events_manager = EventsManager::new(glfw.clone(), window.clone(), receiver);
    
    unsafe { gl::Viewport(0, 0, window_height as i32, window_height as i32); }

    let version = unsafe { std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8) };
    println!("OpenGL version: {}", version.to_str().unwrap());

    unsafe {
      gl::Enable(gl::BLEND);
      gl::Enable(gl::DEPTH_TEST);
      gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    let mut nr_attributes: i32 = 0;
    unsafe { gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut nr_attributes); }
    println!("Maximum number of vertex attributes (input variable for the vertex shader) supported: {} 4-component vertex attributes available", nr_attributes);
    
    Box::new(PhoenixEngine {
      glfw,
      window,
      events_manager,
      layers: Vec::new()
    })
  }

  pub fn run(&mut self) {
    while !self.window.lock().unwrap().should_close() {
      self.events_manager.handle();
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

      self.window.lock().unwrap().swap_buffers();
      self.events_manager.accumulate();
    }
  }
}

/// Temporary function
fn rgba_u8_as_u32(rgba_data: Vec<u8>) -> Vec<u32> {
  rgba_data.chunks(4).map(|rgba| {
      (rgba[0] as u32) << 24 | // Red
      (rgba[1] as u32) << 16 | // Green
      (rgba[2] as u32) << 8  | // Blue
      (rgba[3] as u32)         // Alpha
  }).collect()
}
