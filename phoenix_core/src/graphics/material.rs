use crate::graphics::{
  shaders::ShaderProgram,
  buffers::UniformBufferObject,
  texture::Sampler
};

pub struct Material {
  pub shader: ShaderProgram,
  pub samplers: Vec<Box<dyn Sampler>>,
  pub max_units: usize,
  pub ubo: Option<UniformBufferObject>
}

impl Material {
  pub fn new(
    shader: ShaderProgram,
    units: usize,
    ubo: Option<UniformBufferObject>
  ) -> Material {
    Material {
      shader,
      samplers: Vec::with_capacity(units),
      max_units: units,
      ubo,
    }
  }

  pub fn add_sampler(&mut self, sampler: Box<dyn Sampler>) {
    let uniform = format!("Texture{}", self.samplers.len() as i32);
    println!("[Runtime] Add uniform {} to hashmap", uniform);
    self.shader.create_uniform(&uniform);
    self.shader.set_texture_unit(&uniform, self.samplers.len() as i32);
    if self.samplers.len() >= self.max_units {
      println!("[Runtime] Texture unit limit reach, failed to add sampler to material!");
    } else {
      self.samplers.push(sampler);
      println!("[Runtime] Add sampler to material");
    } 
  }

  pub fn bind(&self) {
    self.shader.activate();
    for (index, sampler) in self.samplers.iter().enumerate() {
      sampler.bind(index as u32);
    }
  }
}
