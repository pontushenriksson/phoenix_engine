use futures::SinkExt;

use crate::graphics::shaders::ShaderProgram;
use crate::graphics::texture::Texture;
use crate::graphics::buffers::UniformBufferObject;

pub struct Material {
  shader: ShaderProgram,
  textures: Vec<Texture>,
  max_units: usize,
  // uniform_data: UniformBufferObject
}

impl Material {
  pub fn new(shader: ShaderProgram, units: usize) -> Material {
    Material {
      shader,
      textures: Vec::with_capacity(units),
      max_units: units,
      // uniform_data: UniformBufferObject::new(),
    }
  }

  pub fn add_texture(&mut self, texture: Texture) {
    self.textures.push(texture);
  }

  pub fn bind(&self) {
    self.shader.activate();
    for (i, texture) in self.textures
      .iter()
      .enumerate()
    {
      texture.bind(i as u32);
    }
  }
}
