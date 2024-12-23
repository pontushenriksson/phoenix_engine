use crate::scenes::scene::*;

pub struct SceneManager {
  // scenes: Vec<Scene>,
  current: usize,
}

/// Handle for 'Scene'
pub struct SceneHandle {

}

impl SceneManager {
  /// Change return value to a SceneHandle later
  pub fn current(&self) -> usize /* SceneHandle */ {
    self.current
  }
}
