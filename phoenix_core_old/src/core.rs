use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use gl;
use glfw::{Context, Glfw, PWindow};
use image::{self, GenericImageView};
use cgmath::*;

use crate::assets::loader::RawVertexData;
use crate::debugger::logger::*;
use crate::graphics::object::StaticGameObject;
use crate::info;
use crate::debugger::debugger::Debugger;
use crate::debugger::debugger::PhoenixError;

use crate::gl_call;
use crate::assets::loader::AssetLoader;
use crate::graphics::camera::PerspectiveCamera;
use crate::graphics::{data::*, shader};
use crate::graphics::mesh::{self, StaticMesh};
use crate::graphics::renderer::Renderer;
use crate::graphics::shader::*;
use crate::graphics::texture::*;
use crate::layers::layer;
// use crate::scenes::scene::{Scene, StaticGameObject};
use crate::window::window::*;
use crate::layers::layer::*;

/// Loading glTF Models
/// For each buffer in a glTF file:

/// Create a VertexBufferObject and upload data.
/// Create VertexAttributeDescriptors for the layout.
/// Use link_vbo or link_separate_vbo based on interleaving.

const TRIANGLE_MODE: bool = true;

pub struct PhoenixCore{
  window_manager: WindowManager,
  renderer: Renderer,
  pub game_layer: Box<GameLayer>,
  ui_layer: Box<UiLayer>,
  logger: Logger,
}

impl PhoenixCore {
  pub fn new(
    window_width: u32,
    window_height: u32,
    title: &str,
    icon_path: &str
  ) -> Result<Box<PhoenixCore>, PhoenixError> {
    println!("--------------------------------- PhoenixEngine ---------------------------------");
    println!("Creating WindowManager ...");
    let window_manager= WindowManager::new(window_width, window_height, title, icon_path);

    println!("Setting viewport ...");

    unsafe { gl::Viewport(0, 0, window_width as i32, window_height as i32); }

    println!("Getting version and shader verion ...");

    let version = unsafe { std::ffi::CStr::from_ptr(gl_call!(gl::GetString(gl::VERSION) as *const i8)) };

    let shader_version = unsafe { std::ffi::CStr::from_ptr(gl::GetString(gl::SHADING_LANGUAGE_VERSION) as *const i8) };

    println!("Info:");

    println!("\tOpenGL version: {}", version.to_str().unwrap());
    println!("\tOpenGL shader version: {}", shader_version.to_str().unwrap());

    unsafe {
      gl_call!(gl::Enable(gl::BLEND));
      gl_call!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA));
      gl_call!(gl::Enable(gl::DEPTH_TEST));
      gl_call!(gl::DepthFunc(gl::LESS));
      gl_call!(gl::Viewport(0, 0, window_width as i32, window_height as i32));
    }

    println!("Info:");

    let mut nr_attributes: i32 = 0;
    unsafe { gl_call!(gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut nr_attributes)); }
    println!("\tMaximum number of vertex attributes (input variable for the vertex shader) supported: {} 4-component (vec4) vertex attributes available", nr_attributes);

    let mut nr_texture_units: i32 = 0;
    unsafe { gl_call!(gl::GetIntegerv(gl::MAX_TEXTURE_IMAGE_UNITS, &mut nr_texture_units)); }
    // Divide batches by this number (modulus with remainder being rendered as a batch as well)
    println!("\tMaximum number of texture units supported: {}", nr_texture_units);

    println!("Packaging and returning PhoenixEngine ...");

    Ok(
      Box::new(
        PhoenixCore {
          window_manager,
          renderer: Renderer::new(),
          game_layer: GameLayer::new(),
          ui_layer: UiLayer::new(),
          logger: Logger::new(),
        }
      )
    )
  }

  pub fn add_static_game_object(&mut self, object: Box<StaticGameObject>) {
    self.game_layer.current_scene().unwrap().add_static_game_object(object);
  }

  pub fn add_shader_program(&mut self, shader: ShaderProgram) {
    self.game_layer.current_scene().unwrap().add_shader_program(shader);
  }

  pub fn run(&mut self) {
    println!("Running ...");

    while !self.window_manager.window.lock().unwrap().should_close() {
      self.window_manager.events_manager.handle();
      self.renderer.clear();

      // Update positions etc.

      self.renderer.render(self.game_layer.current_scene().unwrap());

      self.window_manager.window.lock().unwrap().swap_buffers();
      self.window_manager.events_manager.accumulate();
    }
  }
}

/*

Old loads:

/*

    let texture = Texture::new("C:/dev/phoenix/phoenix_engine/assets/textures/goofy.jpg", TextureType::Diffuse);
    texture.activate(0);

    let vertices = RawVertexData {
      data: vec![
        -0.5, -0.5,  0.0,   0.0, 0.0,
        -0.5,  0.5,  0.0,   0.0, 1.0,
        0.5,  0.5,  0.0,   1.0, 1.0,
        0.5, -0.5,  0.0,   1.0, 0.0,
      ],
      stride: 5, // 3 for position + 2 for texcoord
    };
    
    let indices = Some(vec![0, 2, 1, 0, 3, 2]);
    
    let material = Material {
      r#type: MaterialType::Basic,
      shader: ShaderProgram::new("C:/dev/phoenix/phoenix_engine/shaders/vertex.glsl", "C:/dev/phoenix/phoenix_engine/shaders/fragment.glsl"),
      uniforms: UniformCollection {
        matrices: UniformMatrices {
          model: cgmath::Matrix4::identity(),
          view: cgmath::Matrix4::identity(),
          projection: cgmath::Matrix4::identity(),
        },
        other: std::collections::HashMap::new(),
      },
    };

    */

Old rendering loop:

/*
    let mut elapsed_time = 0.0;
    let mut last_frame = Instant::now();
    let mut frame_count = 0;
    let mut fps_timer = Instant::now();

    let mut camera = PerspectiveCamera::new(cgmath::point3(0.0, 0.0, 3.0), cgmath::point3(0.0, 0.0, 0.0), cgmath::vec3(0.0, 1.0, 0.0), 45.0, (self.window_manager.width / self.window_manager.height) as f32, 0.1, 100.0); // Adjust as necessary
    
    let mut scene = Scene::new();

    scene.static_game_objects.push(StaticGameObject::new(vertices, indices, Some(material)));

    let mut delta_time: f32 = 0.0016;
    */

    while !self.window_manager.window.lock().unwrap().should_close() {
      self.window_manager.glfw.lock().unwrap().poll_events();
      /*
      let now = Instant::now();
      delta_time = (now - last_frame).as_secs_f32();
      last_frame = now;

      // Increment frame count
      frame_count += 1;

      // Every second, calculate and print FPS
      if fps_timer.elapsed() >= Duration::from_secs(1) {
        let fps = frame_count as f32 / fps_timer.elapsed().as_secs_f32();
        print!("\rFPS: {:.2}", fps);
        io::stdout().flush().unwrap();

        // Reset FPS timer and frame count
        fps_timer = Instant::now();
        frame_count = 0;
      }

      elapsed_time += delta_time; // ~6000fps

      let angle = cgmath::Rad(elapsed_time);
      let rotation_matrix = cgmath::Matrix4::from_angle_z(angle);

      camera.position.z -= 1.0 * delta_time;
      camera.update_view_matrix();

      for object in scene.static_game_objects.iter_mut() {
        let angle = cgmath::Rad(elapsed_time);
        let rotation = cgmath::Quaternion::from_axis_angle(Vector3::unit_y(), angle);
        // object.set_transform();
        object.transform.position.x += 0.5 * delta_time;
        object.set_transform(Transform{
          position: object.transform.position,
          rotation,
          scale: cgmath::vec3(1.0, 1.0, 1.0)
        });
      }

      */

      unsafe {
        // Wireframe mode
        // gl_call!(gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE));

        // Regular mode
        gl_call!(gl::PolygonMode(polygon_mode.0, polygon_mode.1));
        
        gl_call!(gl::ClearColor(clear_color[0], clear_color[1], clear_color[2], clear_color[3]));

        gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT));
      }
      
      // Renderer::render(&camera, &scene);

*/
