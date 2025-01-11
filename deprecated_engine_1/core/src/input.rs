use glfw::{Action, WindowEvent, GlfwReceiver, Key};
use std::sync::mpsc::Receiver;

pub struct InputManager {
  // receiver: GlfwReceiver<(f64, glfw::WindowEvent)>,
}

impl InputManager {
  pub fn new(/* receiver: GlfwReceiver<(f64, glfw::WindowEvent)> */) -> InputManager {
    InputManager { /* receiver */ }
  }

  /*

  pub fn receive(&mut self, window: &mut glfw::Window) {
    window.glfw.poll_events();
  }

  pub fn handle(&self, window: &mut glfw::Window, event: glfw::WindowEvent) {
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

  pub fn process(&mut self, window: &mut glfw::Window) {
    for (_, event) in glfw::flush_messages(&self.receiver) {
      self.handle(window, event);
    }
  } 

  */ 
}
