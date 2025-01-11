pub mod vertices;
pub mod indices;
pub mod shaders;
pub mod logging;
pub mod bindings;
pub mod renderer;
pub mod camera;
pub mod scene;
pub mod input;

use shaders::*;
use vertices::*;
use indices::*;
use renderer::*;
// use scene::*;
use input::*;

use glfw::{Glfw, Action, Context, Key, GlfwReceiver, WindowEvent};
use core::ffi::c_void;

pub struct PhoenixEngine {
  glfw: Glfw,
  window: glfw::PWindow,
  events: GlfwReceiver<(f64, WindowEvent)>,

  state: u32,
 
  renderer: Renderer,
  
  /*
  physics: PhysicsEngine,
  */
  input_manager: InputManager,
}

impl PhoenixEngine {
  pub fn new() -> PhoenixEngine {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let (mut window, events) = match glfw.create_window(600, 600, "Phoenix Engine v0.1.0", glfw::WindowMode::Windowed) {
        Some(reciever) => {
            println!("GLFW Window created successfully.\n");
            reciever
        }
        None => {
            panic!("Failed to create GLFW Window.\n");
        }
    };

    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let version = unsafe { std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8) };
    println!("OpenGL version: {}", version.to_str().unwrap());

    PhoenixEngine {
        glfw,
        window,
        events,
        state: 0,
        
        renderer: Renderer::new(),
        input_manager: InputManager::new(),
    }
  }

  pub fn run<F: FnMut()>(&mut self, mut logic: F)  {
    while !self.window.should_close() {
      // self.input_manager.recv();
      // self.physics_system.update();
      // self.renderer.render();

      self.glfw.poll_events();

      for (_, event) in glfw::flush_messages(&self.events) {
        handle_window_event(&mut self.window, event);
      }

      logic();

      

      // self.renderer.render();
      
      // Poll events
      
      self.window.swap_buffers();
    }
  }

  pub fn clean(&mut self) {
    // renderer.clean();
    /*
    
    vbo.unbind();
    vao.unbind();
    ibo.unbind();
    ShaderProgram::unbind(); // Maybe change to shader_program.unbind(); later?

    unsafe {
        gl::DeleteVertexArrays(1, &vao.id);
        gl::DeleteBuffers(1, &vbo.id);
        gl::DeleteBuffers(1, &ibo.id);
        gl::DeleteProgram(shader_program.program_handle);
    }

     */
  }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        glfw::WindowEvent::FramebufferSize(width, height) => {
            unsafe {
                gl::Viewport(0, 0, width, height);
            }
        }
        _ => {}
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

#[cfg(test)]
mod tests {
    use super::shaders::*;
    use super::vertices::*;
    use super::indices::*;
    use super::logging::*;
    
    use colored::*;
    use fern::Dispatch;
    use log::{error, info};
    use gl;
    use gl::types::*;
    use glfw::{Action, Context, Key};
    use std::num;
    use std::mem;
    use std::ffi::c_void;
    use std::ptr;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Read;
    use std::ffi::CString;
    /*
    use glm::*;
    use glm::ext::translate;
    use glm::ext::perspective;
    */
    use cgmath::{Matrix4, Vector3, Rad, perspective, Deg};

    const WINDOW_HEIGHT: u32 = 800;
    const WINDOW_WIDTH: u32 = 800;

    fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true);
            }
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe {
                    gl::Viewport(0, 0, width, height);
                }
            }
            _ => {}
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

    #[test]
    fn simple_engine_test() {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let (mut window, events) = match glfw.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "Phoenix Engine v0.1.0", glfw::WindowMode::Windowed) {
            Some(reciever) => {
                println!("GLFW Window created successfully.\n");
                reciever
            }
            None => {
                panic!("Failed to create GLFW Window.\n");
            }
        };

        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        window.make_current();

        gl::load_with(|s| window.get_proc_address(s) as *const _);

        let version = unsafe { std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8) };
        println!("OpenGL version: {}", version.to_str().unwrap());

        let vertices: [f32; 36] = [
            // Coordinates                                                              // Colors
            -0.5, -0.5, 0.0,    /* Bottom-left */                                          1.0, 0.0, 0.0,
            0.5, -0.5, 0.0,    /* Bottom-right*/                                          0.0, 1.0, 0.0,
            0.0,  1.0, 0.0,    /* Top of the large triangle (move this vertex higher) */  0.0, 0.0, 1.0,
            -0.25, 0.5, 0.0,    /* Left vertex for the top triangle */                     0.9, 0.45, 0.17,
            0.25, 0.5, 0.0,    /* Right vertex for the top triangle */                    0.9, 0.45, 0.17,
            0.0, -0.5, 0.0,    /* Bottom center */                                        0.8, 0.3, 0.02,
        ];

        let indices: [GLuint; 9] = [
            0, 5, 3,  // Bottom-left triangle
            1, 5, 4,  // Bottom-right triangle
            2, 3, 4,  // Top triangle
        ];

        let vao = VertexArrayObject::new();
        vao.bind();

        let vbo = VertexBufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
        vbo.bind();

        vbo.store_f32_data(&vertices);

        let ibo = IndexBufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
        ibo.bind();

        ibo.store_u32_data(&indices);

        let position_attribute = VertexAttribute::new(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * mem::size_of::<GLfloat>() as GLsizei, // aPos (3) + aColor (3) + ...
            0 as *const c_void  // Start from the beginning of the buffer
        );

        let position_attribute2 = VertexAttribute::new(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * mem::size_of::<GLfloat>() as GLsizei, // aPos (3) + aColor (3) + ...
            (3 * mem::size_of::<GLfloat>()) as *const c_void  // Start from the beginning of the buffer
        );

        position_attribute.enable();
        position_attribute2.enable();
        
        let mut shader_program = ShaderProgram::new("C:/dev/phoenix/phoenix_engine/core/src/shaders/shader.vert", "C:/dev/phoenix/phoenix_engine/core/src/shaders/shader.frag");
        shader_program.bind();

        shader_program.create_uniform("scale");
        
        unsafe {
            //gl::Uniform1f(0, -0.5); // "scale".to_string().unwrap()
            gl::Enable(gl::DEPTH_TEST);
        }

        while !window.should_close() {
            // gl::UseProgram(shader_program)
            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                handle_window_event(&mut window, event);
            }

            let (width, height) = window.get_framebuffer_size();

            unsafe {
                gl::Viewport(0, 0, width, height);
                
                gl::ClearColor(0.7, 0.13, 0.17, 0.1);
                check_gl_error();

                gl::Clear(gl::COLOR_BUFFER_BIT);
                check_gl_error();

                gl::Clear(gl::DEPTH_BUFFER_BIT);

                gl::DrawElements(gl::TRIANGLES, 9, gl::UNSIGNED_INT, 0 as *const c_void);
                check_gl_error();

                
            }

            window.swap_buffers();
        }

        vbo.unbind();
        vao.unbind();
        ibo.unbind();
        ShaderProgram::unbind(); // Maybe change to shader_program.unbind(); later?

        unsafe {
            gl::DeleteVertexArrays(1, &vao.id);
            gl::DeleteBuffers(1, &vbo.id);
            gl::DeleteBuffers(1, &ibo.id);
            gl::DeleteProgram(shader_program.program_handle);
        }
    }
}
