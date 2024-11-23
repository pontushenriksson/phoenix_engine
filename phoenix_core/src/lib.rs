use core::PhoenixEngine;
use glfw::WindowEvent;

pub mod core;
pub mod events {
  pub mod events;
}
pub struct PhoenixApplication {
  engine: Box<PhoenixEngine>,
}

impl PhoenixApplication {
  pub fn new(
    window_width: u32,
    window_height: u32,
    title: &str,
    icon_path: &str
  ) -> Box<PhoenixApplication> {
    Box::new(PhoenixApplication {
      engine: PhoenixEngine::new(
        window_width,
        window_height,
        title,
        icon_path  
      )
    })
  }

  pub fn bind_input<F: Fn()>(&mut self, input: u32 /* input: WindowEvent */, func: F) {
    func();
    panic!("Bound function!");
  }

  pub fn run(&mut self) {
    self.engine.run();
  }
}

/*

#[cfg(test)]
mod tests {
    use super::*;

    fn test() {
        
    }
}

*/
