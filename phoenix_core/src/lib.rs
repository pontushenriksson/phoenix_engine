use core::PhoenixEngine;

pub mod core;
pub struct PhoenixApplication {
  engine: PhoenixEngine,
}

impl PhoenixApplication {
  pub fn new() -> PhoenixApplication {
    PhoenixApplication {
        engine: PhoenixEngine {},
    }
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
