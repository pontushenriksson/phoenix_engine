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

  use crate::graphics::buffers::UniformBufferObject;
use crate::graphics::data::{Attribute, VertexDescriptor};
  use crate::graphics::material::Material;
use crate::graphics::mesh::{BufferType, Mesh};
use crate::graphics::shaders::ShaderProgram;
  
  use crate::core::{PhoenixApplication, PhoenixEngineInfo};
use crate::graphics::texture::{Diffuse, Sampler, Sampler2D, Specular};
use crate::objects::lights::PointLight;
use crate::objects::objects::GameObject;
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

    let descriptor = VertexDescriptor {
      attributes: vec![
        Attribute::Vec3,
        Attribute::Vec3,
        Attribute::Vec4,
        Attribute::Vec2,
      ],
      stride: 12,
    };

    let mesh = Mesh::new(
      BufferType::Static,
      vertices.to_vec(),
      Some(indices.to_vec()),
      descriptor,
    );

    let shader = ShaderProgram::new(
      "../assets/materials/planks/shaders/shader.vert",
      "../assets/materials/planks/shaders/shader.frag",
    );

    let diffuse = Sampler2D::<Diffuse>::new("../assets/textures/bricks texture.jpg", 0, gl::RGBA, gl::UNSIGNED_BYTE);
    let specular = Sampler2D::<Specular>::new("../assets/textures/bricks specular.png", 1, gl::RGBA, gl::UNSIGNED_BYTE);

    let u_data = [
      0.42,
      1.2,
      0.3,
    ];

    let ubo = UniformBufferObject::new((u_data.len() * std::mem::size_of::<f32>()) as isize);
    // ubo.set_data(0, &u_data);

    let mut material = Material::new(shader, PhoenixEngineInfo::get_texture_unit_count() as usize, ubo);
    material.add_sampler(diffuse);
    material.add_sampler(specular);
    // material.set_ubo_data();

    let game_object = GameObject::new(mesh, material);
    
    app.add_static_object(game_object);

    app.run();
  }
}
