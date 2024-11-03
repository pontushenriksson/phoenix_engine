use gl;
use glfw::{FlushedMessages, Glfw, GlfwReceiver, Window, Key, WindowEvent, Action};
use rayon::prelude::*;
use std::sync::{mpsc, Arc, Mutex};
use std::collections::HashMap;

pub struct EventHandler {
  pub glfw: Glfw,
  pub receiver: GlfwReceiver<(f64, WindowEvent)>,
  pub actions: HashMap<WindowEvent, Vec<Box<dyn FnMut()>>>,
}

impl EventHandler {
  pub fn new(glfw: Glfw, receiver: GlfwReceiver<(f64, WindowEvent)>) -> EventHandler {
    EventHandler {
      glfw: glfw,
      receiver: receiver,
      actions: HashMap::new(),
    }
  }

  pub fn accumulate(&mut self) {
    self.glfw.poll_events();
  }

  pub fn handle(&mut self, window: &mut glfw::PWindow) {
    let events: Vec<(f64, WindowEvent)> = glfw::flush_messages(&self.receiver).collect();

    events.into_par_iter().for_each(|(_, event)| {
      handle_event(event);
    });
  }

  /*

  pub fn map_action_to_event<F>(&mut self, event: WindowEvent, action: F)
  where 
    F: FnMut() + 'static
  {
    self.actions.entry(event)
      .or_insert_with(Vec::new)
      .push(Box::new(action))
  }

  pub fn trigger(&mut self, event: &WindowEvent) {
    if let Some(actions) = self.actions.get_mut(event) {
      for action in actions {
        action();
      }
    }
  }
  
  */
}



pub fn handle_event(event: WindowEvent) /* -> Result<..., ...>*/ {
  println!("Handling event: {:?}", event);
  
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
}
