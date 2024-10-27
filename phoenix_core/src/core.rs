use gl;
use glfw::{Glfw, Action, Context, Key, GlfwReceiver, WindowEvent};
use image::GenericImageView;
use log::{error, info};
use std::thread;
use std::sync::mpsc;

use crate::events::events;
pub struct PhoenixEngine {
  window: glfw::PWindow, // Window manager
  events: events::Accumulator,
  event_handler: events::Handler,
  // state: u32,
}

impl PhoenixEngine {
  pub fn new(window_width: u32, window_height: u32, title: &str /* "Phoenix Engine v0.1.0" */) -> PhoenixEngine {
    let mut glfw: Glfw = glfw::init(glfw::fail_on_errors).unwrap();

    glfw.window_hint(glfw::WindowHint::Resizable(true));
    glfw.window_hint(glfw::WindowHint::TransparentFramebuffer(true));

    let (mut window, event_receiver) = match glfw.create_window(window_width, window_height, title, glfw::WindowMode::Windowed) {
      Some(reciever) => {
        info!("GLFW Window created successfully.\n");
        reciever
      }
      None => {
        panic!("Failed to create GLFW Window.\n");
      }
    };
    
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let version = unsafe { std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8) };
    println!("OpenGL version: {}", version.to_str().unwrap());

    let icon = image::open("./assets/icons/icon.png").unwrap();
    let (width, height) = icon.dimensions();
    #[allow(unused_variables)]
    let icon_rgba = icon.to_rgba8().as_raw().clone();

    let icon_pixels = rgba_u8_as_u32(icon.to_rgba8().into_raw());

    let glfw_icon = glfw::PixelImage {
      width: width as u32,
      height: height as u32,
      pixels: icon_pixels,
    };

    window.set_icon_from_pixels(vec![glfw_icon]);

    unsafe {
      gl::Enable(gl::DEPTH_TEST);
      gl::Enable(gl::BLEND);
      gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    PhoenixEngine {
      window: window,
    }
  }

  pub fn run<F: FnMut()>(&mut self, mut logic: F) /* -> Result<PhoenixLogPath, Vec<ErrorMessage>> */ {  
    // spawn a thread for event handling
    
    while !self.window.should_close() {
      self.events.accumulate();
      
      logic(/* Takes events as input maybe? */);
    
      unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        gl::Clear(gl::DEPTH_BUFFER_BIT);
        gl::Clear(gl::COLOR_BUFFER_BIT);
      }

      self.window.swap_buffers();
    }

    // .join().unwrap();
  }
}

fn rgba_u8_as_u32(rgba_data: Vec<u8>) -> Vec<u32> {
  rgba_data.chunks(4).map(|rgba| {
      (rgba[0] as u32) << 24 | // Red
      (rgba[1] as u32) << 16 | // Green
      (rgba[2] as u32) << 8  | // Blue
      (rgba[3] as u32)        // Alpha
  }).collect()
}
