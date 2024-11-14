use gl;
use glfw::{Glfw, Action, Context, Key, GlfwReceiver, WindowEvent};
use image::GenericImageView;
use log::{error, info};
use std::thread;
use std::sync::mpsc;
use rayon::prelude::*;
use std::time::{Instant, Duration};
use std::collections::HashMap;
use core::ffi::c_void;
use std::mem;

use crate::events::events;
use crate::graphics::renderer;
use crate::debugger::debugger::*;
use crate::graphics::{self, renderer::*, shaders::ShaderProgram};
use crate::assets::loader;
use crate::ecs::components::Texture2D;

pub struct PhoenixEngine<T, E> {
  debugger: PhoenixDebugger<T, E>,
  window: glfw::PWindow, // Window manager
  renderer: graphics::renderer::PhoenixRenderer,
  events: events::EventHandler,
  last_frame: std::time::Instant,
  delta_time: f32,
  // event_handler: events::Handler,
  // state: u32,
  textures: Vec<Texture2D>, // Change to general 'Texture' later
  shaders: Vec<ShaderProgram>,
  static_objects: Vec<StaticGameObject>

  // render_que: RenderQue,
}

impl<T, E> PhoenixEngine<T, E>
where
    T: std::fmt::Debug,
    E: std::fmt::Debug,
{
  pub fn new(window_width: u32, window_height: u32, title: &str /* "Phoenix Engine v0.1.0" */, icon_path: &str) -> Box<PhoenixEngine<T, E>> {
    let mut glfw: Glfw = glfw::init(glfw::fail_on_errors).unwrap();

    glfw.window_hint(glfw::WindowHint::Resizable(true));
    glfw.window_hint(glfw::WindowHint::TransparentFramebuffer(true));

    let (mut window, receiver) = match glfw.create_window(window_width, window_height, title, glfw::WindowMode::Windowed) {
      Some(receiver) => {
        info!("GLFW Window created successfully.\n");
        receiver
      }
      None => {
        panic!("Failed to create GLFW Window.\n");
      }
    };
    
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|s| window.get_proc_address(s) as *const _);
    
    unsafe { gl::Viewport(0, 0, window_height as i32, window_height as i32); }

    let version = unsafe { std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8) };
    println!("OpenGL version: {}", version.to_str().unwrap());

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

    unsafe {
      gl::Enable(gl::DEPTH_TEST);
      gl::Enable(gl::BLEND);
      gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    let renderer: renderer::PhoenixRenderer = renderer::PhoenixRenderer::new();
    let events: events::EventHandler = events::EventHandler::new(glfw, receiver);

    Box::new(PhoenixEngine {
      debugger: PhoenixDebugger::new(
        DebuggerRunningMode::Accumulate(
          DebuggerOutputMode::Terminal
        )
      ),
      window: window,
      renderer: renderer,
      events: events,
      last_frame: Instant::now(),
      delta_time: 0.0,
      textures: vec![],
      shaders: vec![],
      static_objects: vec![],
    })
  }

  pub fn update(&mut self) {
    let now = std::time::Instant::now();
    let duration = now.duration_since(self.last_frame);
    self.delta_time = duration.as_secs_f32();
    self.last_frame = now;
  }

  pub fn run<F: FnMut()>(&mut self, mut logic: F) /* -> Result<PhoenixLogPath, Vec<ErrorMessage>> */ {  
    self.debugger.set_mode(DebuggerRunningMode::Accumulate(DebuggerOutputMode::File));
    
    let mut nr_attributes: i32 = 0;
    unsafe { gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut nr_attributes); }
    println!("Maximum number of vertex attributes (input variable for the vertex shader) supported: {} 4-component vertex attributes available", nr_attributes);
    self.debugger.info(format!("Maximum number of vertex attributes (input variable for the vertex shader) supported: {} 4-component vertex attributes available", nr_attributes));

    self.debugger.info("ddasda");
    // texture.into_mipmap();
    
    // shader_program.create_uniform("tex0");
    
    // self.shaders.get(0).unwrap().bind(); // Change later to be a selected shader

    while !self.window.should_close() {
      self.events.handle(&mut self.window);

      // self.update(); // Function which changes current data for new updated data

      logic();

      // self.renderer.render();

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

        self.static_objects.get(0).unwrap().render(&self.textures, &self.shaders);
  
        VertexArrayObject::unbind();
      }

      self.window.swap_buffers();
      self.events.accumulate();
    }

    // unbind stuff
    /*
    
    A VAO stores the glBindBuffer calls when the target is GL_ELEMENT_ARRAY_BUFFER. 
    This also means it stores its unbind calls so make sure you don't unbind the element array buffer before unbinding your VAO,
    otherwise it doesn't have an EBO configured. 
    
    */
    // .join().unwrap();
  }
}

/*

pub struct Query<Tuple> {

}

*/

pub enum Resource {
  Shader(ShaderProgram),
  Texture(Texture2D), // Add 1D and 3D later
  // Mesh(Mesh),
  // Component(Component)
}

impl<T, E> PhoenixEngine<T, E>
where
    T: std::fmt::Debug,
    E: std::fmt::Debug,
{
  pub fn create_texture2D(path: &str) -> Resource {
    Resource::Texture(Texture2D::new(path))
  }
  
  pub fn create_shader(vert_path: &str, frag_path: &str) -> Resource {
    Resource::Shader(ShaderProgram::new(vert_path, frag_path))
  }
  
  pub fn add_resource(&mut self, resource: Resource) {
    match resource {
      Resource::Shader(program) => self.shaders.push(program),
      Resource::Texture(texture) => self.textures.push(texture)
      // _ => println!("Resource not implemented!")
    }
  }

  pub fn new_static_object(&mut self, vertices: Vec<f32>, indices: Vec<u32>, texture: usize, shader_program: usize) {
    self.static_objects.push(
      StaticGameObject::new(
        vertices, 
        indices, 
        texture,
        shader_program
      )
    );
  }
}

fn rgba_u8_as_u32(rgba_data: Vec<u8>) -> Vec<u32> {
  rgba_data.chunks(4).map(|rgba| {
      (rgba[0] as u32) << 24 | // Red
      (rgba[1] as u32) << 16 | // Green
      (rgba[2] as u32) << 8  | // Blue
      (rgba[3] as u32)         // Alpha
  }).collect()
}

pub mod bindings {
  pub enum KeyAction {
    KeyPress(char),
    KeyRepeat(char),
    KeyRelease(char),
  }

  pub fn register_action_for_event<F: FnMut()>(action: KeyAction, mut func: F) {
    func();
  }
}

fn check_gl_error() {
  unsafe {
    let error = gl::GetError();
    if error != gl::NO_ERROR {
      println!("OpenGL error: {:?}", error);
    }
  }
}


