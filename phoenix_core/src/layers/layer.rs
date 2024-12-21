pub trait Layer {
  fn on_attach(&self) {

  }

  fn on_update(&self) {

  }

  fn on_event(&self, /* event: &Event */) {
    
  }

  fn on_detach(&self) {
    
  }
}

pub struct LayerHandle(usize);

pub struct LayerStack {
  stack: Vec<Box<dyn Layer>>
}

impl LayerStack {
  pub fn new() -> LayerStack {
    LayerStack {
      stack: Vec::new()
    }
  }

  pub fn push<T: Layer + 'static>(&mut self, layer: Box<T>) -> LayerHandle {
    self.stack.push(layer);
    LayerHandle(self.stack.len() - 1)
  }

  pub fn pop(&mut self) {
    self.stack.pop();
  }

  pub fn get() {
    
  }

  pub fn is_handle_valid(&self, handle: &LayerHandle) -> bool {
    match self.stack.get(handle.0) {
      Some(_) => true,
      None => false
    }
  }
}
