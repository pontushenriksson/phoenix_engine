use gl;
use glfw::{FlushedMessages, Glfw, GlfwReceiver, Window, Key, WindowEvent, Action};
use rayon::prelude::*;
use std::sync::{mpsc, Arc, Mutex};


/*

pub struct EventManager<'a> {
  glfw: &'a mut Glfw,
  receiver: GlfwReceiver<(f64, WindowEvent)>,
  tx: mpsc::Sender<WindowEvent>,
}

impl EventManager<'_> {
  pub fn new<'a>(glfw: &'a mut Glfw, receiver: GlfwReceiver<(f64, WindowEvent)>, tx: mpsc::Sender<WindowEvent>) -> EventManager {
    EventManager {
      glfw: glfw,
      receiver: receiver,
      tx: tx,
    }
  }

  pub fn accumulate(&mut self) {
    self.glfw.poll_events();

    let events: Vec<(f64, WindowEvent)> = glfw::flush_messages(&self.receiver).collect();

    events.into_par_iter().for_each(|(_, event)| {
      Self::handle_event(event);
    });
  }

  fn handle_event(event: WindowEvent) /* -> Result<..., ...>*/ {
    println!("Handling event: {:?}", event);
  }
}

*/

pub struct Accumulator {
  glfw: Glfw,
  receiver: GlfwReceiver<(f64, WindowEvent)>,
}

impl Accumulator {
  pub fn new(glfw: Glfw, receiver: GlfwReceiver<(f64, WindowEvent)>) -> Accumulator {
    Accumulator {
      glfw: glfw,
      receiver: receiver,
    }
  }

  pub fn accumulate(&mut self) {
    self.glfw.poll_events();
  }
}

pub struct Handler<'a> {
  window_handle: &'a mut glfw::PWindow,
}

impl Handler {
  pub fn new() {
    
  }
}
