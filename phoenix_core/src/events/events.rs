use glfw::{Action, Glfw, GlfwReceiver, Key, PWindow, WindowEvent};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
pub struct EventManager {
  glfw: Arc<Mutex<Glfw>>,
  window: Arc<Mutex<PWindow>>,
  receiver: GlfwReceiver<(f64, WindowEvent)>,
  // mapper: EventMapper,
}

impl EventManager {
  pub fn new(
    glfw: Arc<Mutex<Glfw>>,
    window: Arc<Mutex<PWindow>>,
    receiver: GlfwReceiver<(f64, WindowEvent)>,
  ) -> EventManager {
    EventManager { glfw, window, receiver }
  }

  /// Accumulates events using 'glfw.poll_events()'
  pub fn accumulate(&self) {
    self.glfw.lock().unwrap().poll_events();
  }

  /// Handles events
  pub fn handle(&mut self, /* camera: &mut Camera */) {
    let events: Vec<(f64, WindowEvent)> = glfw::flush_messages(&self.receiver).collect();
    // Process events in parallel directly (without spawning a thread)

    events.into_par_iter().for_each(|(ts, event)| {
      handle_event(ts, event, &self.window);
    });
  }
}

pub fn handle_event(_ts: f64, event: WindowEvent, _window: &Arc<Mutex<PWindow>>) {
  // println!("[Runtime] Handling event: \"{:?}\" at {:.3} seconds", event, ts);
  
  static mut RUNNING_MODE: bool = false;

  match event {
    glfw::WindowEvent::Key(Key::Enter, _, Action::Press, _) => {
      unsafe {
        if RUNNING_MODE {
          gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
          RUNNING_MODE = false;
        } else {
          gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
          RUNNING_MODE = true;
        } 
      }
    }
    glfw::WindowEvent::FramebufferSize(width, height) => {
      unsafe {
        gl::Viewport(0, 0, width, height);
      }
    }
    _ => {
      
    }
  }
}
