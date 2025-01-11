use crate::assets::loader::RawVertexData;

use crate::graphics::camera::Camera;
use crate::graphics::object::{StaticGameObject, DynamicGameObject};
use crate::graphics::shader::ShaderProgram;

pub struct Scene {
  pub camera: Box<dyn Camera>, // Vec<Box<dyn Camera>>
  static_game_objects: Vec<Box<StaticGameObject>>,
  dynamic_game_objects: Vec<Box<DynamicGameObject>>,
  shader_programs: Vec<ShaderProgram>
}

impl Scene {
  pub fn new(
    camera: Box<dyn Camera>
  ) -> Box<Scene> {
    Box::new(
      Scene {
        camera,
        static_game_objects: Vec::new(),
        dynamic_game_objects: Vec::new(),
        shader_programs: Vec::new(),
      }
    )
  }

  pub fn add_static_game_object(&mut self, object: Box<StaticGameObject>) -> usize {
    self.static_game_objects.push(object);
    self.static_game_objects.len() - 1
  }

  pub fn remove_static_game_object(&mut self, index: usize) {
    self.static_game_objects.remove(index);
  }

  pub fn add_dynamic_game_object(&mut self, object: Box<DynamicGameObject>) -> usize {
    self.dynamic_game_objects.push(object);
    self.dynamic_game_objects.len() - 1
  }

  pub fn remove_dynamic_game_object(&mut self, index: usize) {
    self.dynamic_game_objects.remove(index);
  }

  pub fn add_shader_program(&mut self, shader: ShaderProgram) -> usize {
    self.shader_programs.push(shader);
    self.shader_programs.len() - 1
  }

  pub fn remove_shader_program(&mut self, index: usize) {
    self.shader_programs.remove(index);
  }

  pub fn static_objects(&self) -> &Vec<Box<StaticGameObject>> {
    &self.static_game_objects
  }

  pub fn get_shader_program(&self, index: usize) -> Option<&ShaderProgram> {
    self.shader_programs.get(index)
  }
} 
