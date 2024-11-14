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
use crate::graphics::{self, renderer::*, shaders::ShaderProgram};
use crate::assets::loader;
use crate::ecs::components::Texture2D;

pub struct PhoenixEngine {
  window: glfw::PWindow, // Window manager
  renderer: graphics::renderer::PhoenixRenderer,
  events: events::EventHandler,
  last_frame: std::time::Instant,
  delta_time: f32,
  // event_handler: events::Handler,
  // state: u32,
}

impl PhoenixEngine {
  pub fn new(window_width: u32, window_height: u32, title: &str /* "Phoenix Engine v0.1.0" */) -> Box<PhoenixEngine> {
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

    let icon = image::open("../assets/icons/icon.png").unwrap();
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
      window: window,
      renderer: renderer,
      events: events,
      last_frame: Instant::now(),
      delta_time: 0.0,
    })
  }

  pub fn update(&mut self) {
    let now = std::time::Instant::now();
    let duration = now.duration_since(self.last_frame);
    self.delta_time = duration.as_secs_f32();
    self.last_frame = now;
  }

  pub fn run<F: FnMut()>(&mut self, mut logic: F) /* -> Result<PhoenixLogPath, Vec<ErrorMessage>> */ {  
    let mut nr_attributes: i32 = 0;
    unsafe { gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut nr_attributes); }
    println!("Maximum number of vertex attributes (input variable for the vertex shader) supported: {} 4-component vertex attributes available", nr_attributes);
    
    let data: [f32; 36] = [
    // Vertices                  Colors                      Texture coordinates
       0.5,  0.5,  0.0,          1.0, 0.0, 0.0, 1.0,         1.0, 1.0, // Top right
       0.5, -0.5,  0.0,          0.0, 1.0, 0.0, 1.0,         1.0, 0.0, // Bottom right
      -0.5, -0.5,  0.0,          0.0, 0.0, 1.0, 1.0,         0.0, 0.0, // Bottom left
      -0.5,  0.5,  0.0,          1.0, 1.0, 0.0, 1.0,         0.0, 1.0, // Top left
    ];

    let indices: [u32; 6] = [
      0, 1, 3,
      1, 2, 3,
    ];

    let vao: VertexArrayObject = VertexArrayObject::new();
    vao.bind();

    let vbo: VertexBufferObject = VertexBufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    
    vbo.store_f32_data(&data);

    let ebo: ElementBufferObject = ElementBufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ebo.bind();

    ebo.store_u32_data(&indices);

    let texture = Texture2D::new("../assets/textures/goofy.jpg");

    texture.bind();

    // texture.into_mipmap();

    let pos_v_attrib: VertexAttribute = VertexAttribute::new(
      0,
      3,
      gl::FLOAT,
      gl::FALSE,
      9 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei, // aPos (vec3) + aColor (vec4) + aTexCoord (vec2)
      0 as *const c_void,
    );

    let color_v_attrib: VertexAttribute = VertexAttribute::new(
      1,
      4,
      gl::FLOAT,
      gl::FALSE,
      9 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei, // aPos (vec3) + aColor (vec4) + aTexCoord (vec2)
      (3 * mem::size_of::<gl::types::GLfloat>()) as *const c_void,
    );

    let texture_v_attrib: VertexAttribute = VertexAttribute::new(
      2,
      2,
      gl::FLOAT,
      gl::FALSE,
      9 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei, // aPos (vec3) + aColor (vec4) + aTexCoord (vec2)
      (7 * mem::size_of::<gl::types::GLfloat>()) as *const c_void,
    );

    pos_v_attrib.enable();
    color_v_attrib.enable();
    texture_v_attrib.enable();

    let mut shader_program: ShaderProgram = ShaderProgram::new("./shaders/default.vert", "./shaders/default.frag");
    
    // shader_program.create_uniform("tex0");

    shader_program.bind();

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
  
        vao.bind();
        shader_program.bind();
  
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null()); // Last arg is an offset or index array (when not using indices)
        check_gl_error();
  
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

fn display() {
  /*
  gl::Clear(gl::DEPTH_BUFFER_BIT);
  check_gl_error();

  gl::Clear(gl::COLOR_BUFFER_BIT);
  check_gl_error();
  */

  // self.window.swap_buffers();
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
