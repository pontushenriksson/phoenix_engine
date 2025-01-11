use cgmath::*;

use crate::graphics::mesh::StaticMesh;
use crate::graphics::mesh::DynamicMesh;

pub struct Transform {
  pub translation: cgmath::Vector3<f32>,
  pub rotation: cgmath::Quaternion<f32>,
  pub scale: cgmath::Vector3<f32>,
}

impl Transform {
  pub fn none() -> Transform {
    Transform {
      translation: cgmath::Vector3::new(0.0, 0.0, 0.0),
      rotation: cgmath::Quaternion::new(1.0, 1.0, 1.0, 1.0), // Identity
      scale: cgmath::Vector3::new(1.0, 1.0, 1.0)
    }
  }

  pub fn matrix(&self) -> cgmath::Matrix4<f32> {
    // Translation
    let translation = cgmath::Matrix4::from_translation(self.translation);
    // Rotation
    let rotation = cgmath::Matrix4::from(self.rotation);
    // Scale
    let scale = cgmath::Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);

    // Transformation
    translation * rotation * scale
  }

  pub fn rotate(&mut self, axis: cgmath::Vector3<f32>, angle: f32) {
    let rotation = cgmath::Quaternion::from_axis_angle(axis.normalize(), Rad(angle));
    self.rotation = rotation * self.rotation;
  }
}

pub struct StaticGameObject {
  pub mesh: Box<StaticMesh>,
  pub transform: Transform,
}

impl StaticGameObject {
  pub fn from(mesh: Box<StaticMesh>, transform: Transform) -> Box<StaticGameObject> {
    Box::new(
      StaticGameObject {
        mesh,
        transform
      }
    )
  }
}

pub struct DynamicGameObject {
  pub mesh: DynamicMesh,
  pub transform: Transform,
}
