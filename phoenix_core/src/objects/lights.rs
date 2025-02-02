use crate::{core::PhoenixEngineInfo, graphics::{camera::Camera, material::Material, shaders::ShaderProgram}, objects::{geometry::Quad, transform::Transform}};

pub struct PointLight {
  pub quad: Quad,
  pub material: Material,
  // pub position: cgmath::Vector3<f32>,
  pub transform: Transform,
  pub color: cgmath::Vector3<f32>,
  pub intensity: f32,
  pub attenuation: (f32, f32, f32), // (Constant, Linear, Quadratic)
}

impl PointLight {
  pub fn new(
    // position: cgmath::Vector3<f32>,
    transform: Transform,
    color: cgmath::Vector3<f32>,
    intensity: f32,
    attenuation: (f32, f32, f32)
  ) -> PointLight {
    PointLight {
      quad: Quad::new(),
      material: Material::new(
        ShaderProgram::new(
          "../assets/stand-alone shaders/bulb.vert",
          "../assets/stand-alone shaders/bulb.frag"
        ),
        PhoenixEngineInfo::get_texture_unit_count() as usize,
        None
      ),
      // position,
      transform,
      color,
      intensity,
      attenuation,
    }
  }

  pub fn update_matrix(&mut self) {
    let translation = cgmath::Matrix4::from_translation(self.transform.translation);
    let rotation = cgmath::Matrix4::from(self.transform.rotation);
    let scale = cgmath::Matrix4::from_nonuniform_scale(
      self.transform.scale.x,
      self.transform.scale.y,
      self.transform.scale.z,
    );

    self.quad.matrix = translation * rotation * scale;
  }

  pub fn draw(&self, camera: &Camera) {
    self.material.bind();

    self.material.shader.set_matrix4_f32("uModel", &self.quad.matrix);
    self.material.shader.set_matrix4_f32("uView", &camera.view);
    self.material.shader.set_matrix4_f32("uProjection", &camera.projection);
    self.quad.mesh.draw();
  }
}
