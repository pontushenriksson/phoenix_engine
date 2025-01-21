use glfw::{Action, Glfw, GlfwReceiver, Key, PWindow, WindowEvent};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use crate::graphics::camera::Camera;

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
      handle_event(ts, event);
    });
  }
}

pub fn handle_event(ts: f64, event: WindowEvent) {
  println!("[Runtime] Handling event: \"{:?}\" at {:.3} seconds", event, ts);

  match event {
    glfw::WindowEvent::Key(Key::Enter, _, Action::Press, _) => {
      unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
      }
    }
    glfw::WindowEvent::Key(Key::Enter, _, Action::Repeat, _) => {
      unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
      }
    }
    glfw::WindowEvent::Key(Key::Enter, _, Action::Release, _) => {
      unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
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
