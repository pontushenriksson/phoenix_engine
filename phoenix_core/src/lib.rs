pub mod core;

pub mod graphics {
  pub mod buffers;
  pub mod data;
  pub mod material;
  pub mod mesh;
  pub mod shaders;
  pub mod texture;
  pub mod window;
}

pub mod objects {
  pub mod geometry;
  pub mod lights;
  pub mod objects;
  pub mod transform;
}

#[cfg(test)]
mod tests {
  use tokio;

  use crate::graphics::data::Attribute;
  use crate::graphics::mesh::{Mesh, StaticMesh};
use crate::graphics::shaders::ShaderProgram;
  
  use crate::core::PhoenixApplication;
use crate::graphics::texture::{Diffuse, Sampler, Sampler2D, Specular};
use crate::objects::lights::PointLight;
use crate::objects::objects::StaticObject;
use crate::objects::transform::Transform;

  #[tokio::test]
  async fn phoenix_core() {
    let mut app = PhoenixApplication::new(
      1280,
      720,
      "Test Game lib.rs",
      "../assets/icons/icon.png",
    ).unwrap();

    let vertices: [gl::types::GLfloat; 60] = [
    // Coordinates            Normals                        Colors                            Texture Coordinates (if higher than 1.0, texture repeats)
      -0.5, 0.0,  0.5,       -0.577, -0.577,  0.577,         0.83, 0.70, 0.44, 1.0,            0.0, 0.0,
      -0.5, 0.0, -0.5,       -0.577, -0.577, -0.577,         0.83, 0.70, 0.44, 1.0,            1.0, 0.0,
       0.5, 0.0, -0.5,        0.577, -0.577, -0.577,         0.83, 0.70, 0.44, 1.0,            0.0, 0.0,
       0.5, 0.0,  0.5,        0.577, -0.577,  0.577,         0.83, 0.70, 0.44, 1.0,            1.0, 0.0,
       0.0, 0.75, 0.0,        0.000,  0.894,  0.000,         0.92, 0.86, 0.76, 1.0,            0.5, 1.0,
    ];

    let indices: [gl::types::GLuint; 18] = [
      0, 1, 2,
      0, 2, 3,
      0, 1, 4,
      1, 2, 4,
      2, 3, 4,
      3, 0, 4,
    ];

    let attributes: [Attribute; 4] = [
      Attribute::Vec3,
      Attribute::Vec3,
      Attribute::Vec4,
      Attribute::Vec2,
    ];

    let texture = Sampler2D::<Diffuse>::new("../assets/textures/bricks texture.jpg", 0, gl::RGBA, gl::UNSIGNED_BYTE);
    let specular = Sampler2D::<Specular>::new("../assets/textures/bricks specular.png", 1, gl::RGBA, gl::UNSIGNED_BYTE);



    app.run();
  }
}
