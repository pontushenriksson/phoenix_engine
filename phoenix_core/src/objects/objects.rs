use cgmath::SquareMatrix;

use crate::{graphics::{camera::Camera, material::Material, mesh::Mesh}, objects::transform::Transform};

pub struct GameObject {
  pub mesh: Mesh<gl::types::GLfloat, gl::types::GLuint>,
  pub material: Material,
  pub transform: Transform,
  pub matrix: cgmath::Matrix4<f32>,
}

impl GameObject {
  pub fn new(
    mesh: Mesh<gl::types::GLfloat, gl::types::GLuint>,
    material: Material,
  ) -> GameObject {
    GameObject {
      mesh,
      material,
      transform: Transform::identity(),
      matrix: cgmath::Matrix4::identity()
    }
  }

  pub fn with_transform(mut self, transform: Transform) -> GameObject {
    self.transform = transform;

    let translation = cgmath::Matrix4::from_translation(self.transform.translation);
    let rotation = cgmath::Matrix4::from(self.transform.rotation);
    let scale = cgmath::Matrix4::from_nonuniform_scale(
      self.transform.scale.x,
      self.transform.scale.y,
      self.transform.scale.z,
    );

    self.matrix = translation * rotation * scale; // Apply transformations in the correct order

    self
  }

  pub fn update_matrix(&mut self) {
    let translation = cgmath::Matrix4::from_translation(self.transform.translation);
    let rotation = cgmath::Matrix4::from(self.transform.rotation);
    let scale = cgmath::Matrix4::from_nonuniform_scale(
      self.transform.scale.x,
      self.transform.scale.y,
      self.transform.scale.z,
    );

    self.matrix = translation * rotation * scale;
  }

  pub fn draw(&self, camera: &Camera) {
    self.material.bind();
    self.material.shader.set_matrix4_f32("uModel", &self.matrix);
    self.material.shader.set_matrix4_f32("uView", &camera.view);
    self.material.shader.set_matrix4_f32("uProjection", &camera.projection);
    self.mesh.draw();
  }
}
