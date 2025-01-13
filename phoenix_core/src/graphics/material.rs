use crate::graphics::{
  shaders::ShaderProgram,
  buffers::UniformBufferObject,
  texture::Sampler
};

pub struct Material {
  pub shader: ShaderProgram,
  pub samplers: Vec<Box<dyn Sampler>>,
  pub max_units: usize,
  pub ubo: UniformBufferObject
}

impl Material {
  pub fn new(
    shader: ShaderProgram,
    units: usize,
    ubo: UniformBufferObject
  ) -> Material {
    Material {
      shader,
      samplers: Vec::with_capacity(units),
      max_units: units,
      ubo,
    }
  }

  pub fn add_sampler(&mut self, sampler: Box<dyn Sampler>) {
    self.samplers.push(sampler);
  }

  pub fn bind(&self) {
    self.shader.activate();
    for (i, sampler) in self.samplers
      .iter()
      .enumerate()
    {
      sampler.bind();
    }
  }
}
