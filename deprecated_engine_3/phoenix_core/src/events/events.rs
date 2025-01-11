use glfw::{Glfw, GlfwReceiver, PWindow, WindowEvent, Action};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub struct EventsManager/* <F: Fn()> */{
  glfw: Arc<Mutex<Glfw>>,
  window: Arc<Mutex<PWindow>>,
  receiver: GlfwReceiver<(f64, WindowEvent)>,
  // actions: HashMap<WindowEvent, F>
}

impl EventsManager {
  pub fn new(
    glfw: Arc<Mutex<Glfw>>,
    window: Arc<Mutex<PWindow>>,
    receiver: GlfwReceiver<(f64, WindowEvent)>
  ) -> EventsManager {
    EventsManager { glfw, window, receiver }
  }

  /// Accumulates events using 'glfw.poll_events()'
  pub fn accumulate(&mut self) {
    let mut glfw = self.glfw.lock().unwrap();
    glfw.poll_events();
  }

  /// Handles events
  pub fn handle(&mut self) {
    let events: Vec<(f64, WindowEvent)> = glfw::flush_messages(&self.receiver).collect();
    // Process events in parallel directly (without spawning a thread)

    events.into_par_iter().for_each(|(ts, event)| {
      handle_event(ts, event);
    });
  }
}

pub fn handle_event(ts: f64, event: WindowEvent) {
  println!("Handling event: {:?} at {:.3} seconds", event, ts);
  
  /*

  match event {
    glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => {
      
    }
    glfw::WindowEvent::FramebufferSize(width, height) => {
      unsafe {
        gl::Viewport(0, 0, width, height);
      }
    }
    _ => {

    }
  }

  */
}
