use crate::{graphics::{material::Material, mesh::Mesh}, objects::transform::Transform};

pub struct GameObject {
  pub mesh: Mesh<gl::types::GLfloat, gl::types::GLuint>,
  pub material: Material,
  pub transform: Transform,
}

impl GameObject {
  pub fn new(
    mesh: Mesh<gl::types::GLfloat, gl::types::GLuint>,
    material: Material,
  ) -> GameObject {
    GameObject {
      mesh,
      material,
      transform: Transform::identity()
    }
  }

  pub fn with_transform(&mut self, transform: Transform) {
    self.transform = transform
  }
}
