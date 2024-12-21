use std::sync::{Arc, Mutex};
use gl;
use glfw::{Context, Glfw, GlfwReceiver, PWindow, WindowEvent};
use image::{self, GenericImageView};

pub struct WindowManager {
  pub glfw: Arc<Mutex<Glfw>>,
  pub window: Arc<Mutex<PWindow>>,
  pub receiver: GlfwReceiver<(f64, WindowEvent)>,
  pub width: u32,
  pub height: u32
}

impl WindowManager {
  pub fn new(
    window_width: u32,
    window_height: u32,
    title: &str,
    icon_path: &str
  ) -> WindowManager {
    println!("--------------------------------- WindowManager ---------------------------------");
    println!("Initializing glfw ...");
    let glfw = Arc::new(Mutex::new(glfw::init(glfw::fail_on_errors).unwrap()));
    
    glfw.lock().unwrap().window_hint(glfw::WindowHint::Resizable(true));
    glfw.lock().unwrap().window_hint(glfw::WindowHint::TransparentFramebuffer(true));
    glfw.lock().unwrap().window_hint(glfw::WindowHint::Decorated(true));

    // glfw.lock().unwrap().window_hint(/* glfw::Version { major: 4, minor: 6, patch: ?} */);
    // This does not work for some reason, keep commented out for now:
    // glfw.lock().unwrap().window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    println!("Creating window and receiver ...");

    let (mut window, receiver) = match glfw
      .lock().unwrap()
      .create_window(window_width, window_height, title, glfw::WindowMode::Windowed)
    {
      Some((win, rec)) => (win, rec),
      None => { panic!("Failed to create GLFW window!"); }
    };

    window.make_current();
    window.set_all_polling(true);
    
    println!("Loading window icon: {} ...", icon_path);

    let icon = match image::open(icon_path) {
      Ok(icon) => icon,
      Err(err) => { panic!("[Error] Failed to open path to icon {}\n\terr| {}", icon_path, err); }
    };

    let (width, height) = icon.dimensions();
    let icon_pixels = rgba_u8_as_u32(icon.to_rgba8().into_raw());
    let glfw_icon = glfw::PixelImage {
      width: width as u32,
      height: height as u32,
      pixels: icon_pixels
    };

    println!("Setting window icon ...");

    window.set_icon_from_pixels(vec![glfw_icon]);

    println!("Loading gl ...");

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let window = Arc::new(Mutex::new(window));

    println!("Packaging and returning WindowManager ...");
    println!("---------------------------------------------------------------------------------");

    /*
    unsafe {
      gl::Viewport(0, 0, width as i32, height as i32);
    }
    */

    WindowManager {
      glfw,
      window,
      receiver,
      width: window_width,
      height: window_height
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
