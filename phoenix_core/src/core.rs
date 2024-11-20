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
use cgmath;

use crate::events::events;
use crate::graphics::renderer;
use crate::debugger::debugger::*;
use crate::graphics::{self, renderer::*, shaders::ShaderProgram};
use crate::assets::loader;
use crate::ecs::components::Texture2D;

// Move later
use chrono::Local;
use colored::*;

use sysinfo::System;
#[cfg(feature = "gpu_monitoring")]
use nvml_wrapper::Nvml;
#[cfg(feature = "gpu_monitoring")]
use amdgpu::{DeviceHandle, DrmCard};

#[derive(Debug)]
pub struct PhoenixEngine {
  system: System,
  #[cfg(feature = "gpu_monitoring")]
  nvml: Option<Nvml>,
  #[cfg(feature = "gpu_monitoring")]
  amdgpu_device: Option<DeviceHandle>,

  debugger: EngineDebugger,
  window: glfw::PWindow, // Window manager
  renderer: graphics::renderer::PhoenixRenderer,
  events: events::EventHandler,
  last_frame: std::time::Instant,
  operating_time: Instant,
  delta_time: f32,
  // event_handler: events::Handler,
  // state: u32,
  textures: Vec<Texture2D>, // Change to general 'Texture' later
  shaders: Vec<ShaderProgram>,
  static_objects: Vec<StaticGameObject>,
  cameras_3d: Vec<Camera3D>,
  // render_que: RenderQue,
}

impl PhoenixEngine {
  pub fn new(window_width: u32, window_height: u32, title: &str /* "Phoenix Engine v0.1.0" */, icon_path: &str, dbg_mode: DebuggerRunningMode) -> Box<PhoenixEngine> {
    let mut system = System::new_all();
    system.refresh_all();

    #[cfg(feature = "gpu_monitoring")]
    let nvml = Nvml::init().ok();

    #[cfg(feature = "gpu_monitoring")]
    let amdgpu_device = DrmCard::primary().ok().and_then(|card| {
        DeviceHandle::open(card).ok()
    });
    
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

    let mut nr_attributes: i32 = 0;
    unsafe { gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut nr_attributes); }
    println!("Maximum number of vertex attributes (input variable for the vertex shader) supported: {} 4-component vertex attributes available", nr_attributes);
    
    Box::new(PhoenixEngine {
      system,
      #[cfg(feature = "gpu_monitoring")]
      nvml,
      #[cfg(feature = "gpu_monitoring")]
      amdgpu_device,
      debugger: EngineDebugger::new(dbg_mode), 
      window: window,
      renderer: renderer,
      events: events,
      last_frame: Instant::now(),
      operating_time: Instant::now(),
      delta_time: 0.0,
      textures: Vec::new(),
      shaders: Vec::new(),
      static_objects: Vec::new(),
      cameras_3d: Vec::new()
    })
  }

  /// Check both system RAM and GPU memory.
  pub fn check_mem(&mut self) {
    self.check_ram();
    self.check_gpu();
  }

  /// Check system RAM usage.
  pub fn check_ram(&mut self) {
    self.system.refresh_memory();

    let total_memory = self.system.total_memory();
    let used_memory = self.system.used_memory();

    // Printing in KB
    println!("RAM Usage: {:.2} KB / {:.2} KB", used_memory as f64 / 1024.0, total_memory as f64 / 1024.0);

    // Printing in MB
    println!("RAM Usage: {:.2} MB / {:.2} MB", used_memory as f64 / 1048576.0, total_memory as f64 / 1048576.0);

    // Printing in GB
    println!("RAM Usage: {:.2} GB / {:.2} GB", used_memory as f64 / 1073741824.0, total_memory as f64 / 1073741824.0);

    if used_memory > total_memory * 8 / 10 {
      self.debugger.log(LogLevel::Warning, "High ram usage!".to_string());
    }
  }

  /// Check GPU memory usage (NVIDIA and AMD).
  pub fn check_gpu(&self) {
    #[cfg(feature = "gpu_monitoring")]
    {
        if let Some(nvml) = &self.nvml {
            match nvml.device_by_index(0).and_then(|device| device.memory_info()) {
                Ok(memory_info) => {
                    let total_gpu_memory = memory_info.total / 1024 / 1024;
                    let used_gpu_memory = memory_info.used / 1024 / 1024;

                    println!(
                        "NVIDIA GPU Memory Usage: {} MB / {} MB",
                        used_gpu_memory, total_gpu_memory
                    );

                    if used_gpu_memory > total_gpu_memory * 8 / 10 {
                        println!("Warning: High NVIDIA GPU memory usage!");
                    }
                }
                Err(err) => eprintln!("Failed to query NVIDIA GPU memory: {:?}", err),
            }
        }

        if let Some(amdgpu_device) = &self.amdgpu_device {
            if let Ok(vram_usage) = amdgpu_device.memory_info() {
                let total_vram = vram_usage.vram_total / 1024 / 1024;
                let used_vram = vram_usage.vram_used / 1024 / 1024;

                println!(
                    "AMD GPU Memory Usage: {} MB / {} MB",
                    used_vram, total_vram
                );

                if used_vram > total_vram * 8 / 10 {
                    println!("Warning: High AMD GPU memory usage!");
                }
            } else {
                println!("Failed to query AMD GPU memory usage.");
            }
        }

        if self.nvml.is_none() && self.amdgpu_device.is_none() {
            println!("No supported GPU monitoring available.");
        }
    }

    #[cfg(not(feature = "gpu_monitoring"))]
    {
        println!("GPU monitoring is disabled or unsupported.");
    }
  }

  pub fn update(&mut self) {
    let now = std::time::Instant::now();
    let duration = now.duration_since(self.last_frame);
    self.delta_time = duration.as_secs_f32();
    self.last_frame = now;
  }

  pub fn run<F: FnMut()>(&mut self, mut logic: F) /* -> Result<PhoenixLogPath, Vec<ErrorMessage>> */ {  
    // spawn thread with: // self.update(); // Function which changes current data for new updated data

    while !self.window.should_close() {
      self.events.handle(&mut self.window);
      logic();

      self.cameras_3d.get_mut(0).unwrap().inputs(&mut self.window);
      self.cameras_3d.get_mut(0).unwrap().update_matrix(45.0, 0.1, 100.0);
      self.cameras_3d.get_mut(0).unwrap().matrix(&mut self.shaders.get_mut(0).unwrap(), "camMatrix");

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

    self.check_ram();
    self.check_gpu();

    if let Some(flushed_logs) = self.debugger.flush() {
      // println!("Flushed logs: {:?}", flushed_logs);
      
      if self.debugger.mode == DebuggerRunningMode::Instant(DebuggerOutputMode::File)
      || self.debugger.mode == DebuggerRunningMode::Accumulate(DebuggerOutputMode::File) {
        for entry in flushed_logs {
          match &entry.level {
            LogLevel::Info => {
              println!("{} {} {}", "[INFO]".on_green(), "timestamp".green(), entry.message.green().bold());
              self.debugger.write_to_file(format!("{} {} {}", "[INFO]", "date", entry.message))
            }
            LogLevel::Warning => {
              println!("{} {} {}", "[WARNING]".on_magenta(), "timestamp".magenta(), entry.message.magenta().bold());
              self.debugger.write_to_file(format!("{} {} {}", "[WARNING]", "date", entry.message));
            }
            LogLevel::Error => {
              println!("{} {} {}", "[ERROR]".on_red(), "timestamp".red(), entry.message.red().bold());
              self.debugger.write_to_file(format!("{} {} {}", "[ERROR]", "date", entry.message));
            }
            LogLevel::Trace => {
              println!("{} {} {}", "[TRACE]".on_blue(), "timestamp".blue(), entry.message.blue().bold());
              self.debugger.write_to_file(format!("{} {} {}", "[TRACE]", "date", entry.message));
            }
          }
        }
      }
    }

    ShaderProgram::unbind(); // Maybe change to shader_program.unbind(); later?

    /* Move this later into member specific functions

    unsafe {
        gl::DeleteVertexArrays(1, &vao.id);
        gl::DeleteBuffers(1, &vbo.id);
        gl::DeleteBuffers(1, &ibo.id);
        gl::DeleteProgram(shader_program.program_id);
        gl::DeleteTextures(1, texture.id as *const GLuint);
    }

    */
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

impl PhoenixEngine {
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

  pub fn world_space_point_3d(x: f32, y: f32, z: f32) -> cgmath::Point3<f32> {
    cgmath::point3(x, y, z)
  }

  pub fn new_camera_3d(&mut self, width: u32, height: u32, position: cgmath::Point3<f32>) {
    self.cameras_3d.push(
      Camera3D::new(
        width as i32,
        height as i32,
        position,
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

fn check_gl_error(place: &str) {
  unsafe {
    let error = gl::GetError();
    if error != gl::NO_ERROR {
      println!("OpenGL error at {} : {:?}", place, error);
    }
  }
}


