use cgmath::One;

#[derive(Clone)]
pub struct Transform {
  translation: cgmath::Vector3<f32>,
  rotation: cgmath::Quaternion<f32>,
  scale: cgmath::Vector3<f32>,
}

impl Transform {
  pub fn identity() -> Transform {
    Transform {
      translation: cgmath::vec3(0.0, 0.0, 0.0),
      rotation: cgmath::Quaternion::one(),
      scale: cgmath::vec3(1.0, 1.0, 1.0),
    }
  }
}
