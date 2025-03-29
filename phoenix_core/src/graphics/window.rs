use crate::gl_call;
use crate::debugger::debugger::Debugger;

use std::sync::{Arc, Mutex};
use glfw::{Context, Glfw, PWindow, WindowEvent};
use image::{self, GenericImageView};

use crate::{events::events::EventManager};

pub struct Window {
  pub glfw: Arc<Mutex<Glfw>>,
  pub window: Arc<Mutex<PWindow>>,
  pub event_manager: EventManager,
  pub width: u32,
  pub height: u32,
}

impl Window {
  pub fn new(
    width: u32,
    height: u32,
    title: &str,
    icon: &str,
  ) -> Window {
    println!("------------------------------------ Window -------------------------------------");
    println!("Initializing glfw ...");
    let glfw = Arc::new(Mutex::new(glfw::init(glfw::fail_on_errors).unwrap()));
    
    glfw.lock().unwrap().window_hint(glfw::WindowHint::Resizable(true));
    glfw.lock().unwrap().window_hint(glfw::WindowHint::Decorated(true));

    println!("Initializing window ...");

    let (mut window, receiver) = match glfw
      .lock()
      .unwrap()
      .create_window(width, height, title, glfw::WindowMode::Windowed)
    {
      Some((win, rec)) => (win, rec),
      None => {
        panic!("Failed to create GLFW window!");
      }
    };

    window.make_current();
    window.set_all_polling(true);

    println!("Loading window icon: {} ...", icon);

    let icon = match image::open(icon) {
      Ok(icon) => icon,
      Err(err) => {
        panic!("Failed to open icon: {}\nerr| {}", icon, err);
      }
    };

    let (width, height) = icon.dimensions();
    let icon_pixels = icon.to_rgba8()
      .into_raw()
      .chunks(4)
      .map(
        |rgba| {
          (rgba[0] as u32) << 24 | // Red
          (rgba[1] as u32) << 16 | // Green
          (rgba[2] as u32) << 8  | // Blue
          (rgba[3] as u32)         // Alpha
        }
      ).collect();
    
    let glfw_icon = glfw::PixelImage {
      width: width as u32,
      height: height as u32,
      pixels: icon_pixels
    };

    window.set_icon_from_pixels(vec![glfw_icon]);

    println!("Loading OpenGL ...");

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let window = Arc::new(Mutex::new(window));

    // let event_manager = EventManager::new(glfw.clone(), window.clone(), receiver);
  
    unsafe {
      gl_call!(gl::Enable(gl::BLEND));
      gl_call!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA));
      gl_call!(gl::Enable(gl::DEPTH_TEST));

      // Face Culling (Optimization)
      gl_call!(gl::Enable(gl::CULL_FACE));
      gl_call!(gl::CullFace(gl::FRONT));
      gl_call!(gl::FrontFace(gl::CCW));
      
      // Set clearing color
      gl_call!(gl::ClearColor(0.0, 0.0, 0.0, 1.0));
      gl_call!(gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL));
    }

    let event_manager = EventManager::new(
      glfw.clone(),
      window.clone(),
      receiver,
    );

    println!("Created Window");

    Window {
      glfw,
      window,
      event_manager,
      width,
      height,
    }
  }
}
