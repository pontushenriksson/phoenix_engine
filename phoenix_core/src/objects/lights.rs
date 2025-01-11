 use crate::{graphics::texture::{Diffuse, Sampler, Sampler2D}, objects::geometry::Quad};

pub struct PointLight {
  pub icon: Quad,
  pub position: cgmath::Vector3<f32>,
  pub color: cgmath::Vector3<f32>,
  pub intensity: f32,
  pub attenuation: (f32, f32, f32), // (Constant, Linear, Quadratic)
}

impl PointLight {
  pub fn new(
    position: cgmath::Vector3<f32>,
    color: cgmath::Vector3<f32>,
    intensity: f32,
    attenuation: (f32, f32, f32)
  ) -> PointLight {
    PointLight {
      icon: Quad::new(
        Some(
          Box::new(
            Sampler2D::<Diffuse>::new(
              "../assets/icons/light bulb.png",
              gl::TEXTURE_2D,
              gl::RGBA,
              gl::UNSIGNED_BYTE
            )
          )
        )
      ),
      position,
      color,
      intensity,
      attenuation,
    }
  }
}
