use crate::scenes::scene::Scene;

pub struct GameLayer {
  scenes: Vec<Box<Scene>>,
  current_scene: usize,
}

impl GameLayer {
  pub fn new() -> Box<GameLayer> {
    Box::new(
      GameLayer {
        scenes: Vec::new(),
        current_scene: 0,
      }
    )
  }

  pub fn push_scene(&mut self, scene: Box<Scene>) {
    self.scenes.push(scene);
    self.current_scene = self.scenes.len() - 1;
  }

  pub fn pop_scene(&mut self) {
    self.scenes.pop();
    self.current_scene = self.scenes.len() - 1;
  }

  pub fn get_scene(&self, index: usize) -> Option<&Box<Scene>> {
    self.scenes.get(index)
  }

  pub fn current_scene(&mut self) -> Option<&mut Box<Scene>> {
    self.scenes.get_mut(self.current_scene)
  }
}

pub struct UiLayer {
  // egui context ?
  // egui painter ?
}

impl UiLayer {
  pub fn new() -> Box<UiLayer> {
    // TODO:
    Box::new(
      UiLayer {

      }
    )
  }
}

/*


pub struct LayerHandle(usize);

pub struct LayerStack {
  layers: Vec<GameLayer>,
  overlays: Vec<UiLayer>,
}

impl LayerStack {
  pub fn new() -> LayerStack {
    LayerStack {
      layers: Vec::new(),
      overlays: Vec::new()
    }
  }
}

*/

