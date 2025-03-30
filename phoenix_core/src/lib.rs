pub mod core;

pub mod debugger {
    pub mod debugger;
}

pub mod events {
    pub mod events;
}

pub mod graphics {
    pub mod buffers;
    pub mod camera;
    pub mod data;
    pub mod material;
    pub mod mesh;
    pub mod renderer;
    pub mod shaders;
    pub mod texture;
    pub mod window;

    // Re-exports to simplify imports in games
    pub use buffers::*;
    pub use camera::Camera;
    pub use data::*;
    pub use material::*;
    pub use mesh::*;
    pub use shaders::*;
    pub use texture::*;
}

pub mod objects {
    pub mod geometry;
    pub mod lights;
    pub mod objects;
    pub mod transform;

    // Re-exports to simplify imports in games
    pub use geometry::Ground;
    pub use lights::PointLight;
    pub use objects::GameObject;
    pub use transform::Transform;
}

// Re-export the main application struct at the top level
pub use core::PhoenixApplication;

pub enum Cell<T> {
  Alive(T),
  Dead
}

impl<T> Cell<T> {
  pub fn alive(&mut self) -> &mut T {
    match self {
      Cell::Dead => {
        panic!("Failed alive unwrap on dead cell! at {} {}", file!(), line!());
      }
      Cell::Alive(cell) => cell
    }
  }
}

#[cfg(test)]
mod tests {
  use cgmath::One;
use tokio;

  use crate::graphics::buffers::UniformBufferObject;
  use crate::graphics::data::{Attribute, VertexDescriptor};
  use crate::graphics::material::Material;
  use crate::graphics::mesh::{BufferType, Mesh};
  use crate::graphics::shaders::ShaderProgram;
  
  use crate::core::PhoenixApplication;
  use crate::graphics::texture::{Diffuse, Topography, Sampler, Sampler2D, Specular};
  use crate::objects::geometry::Ground;
  use crate::objects::objects::GameObject;
  use crate::objects::lights::PointLight;
  use crate::graphics::camera::Camera;
  use crate::objects::transform::Transform;

  #[tokio::test]
  async fn phoenix_core() {
    let mut app = PhoenixApplication::new(
      1280,
      720,
      // 1920,
      // 1080,
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
      stride: 3 + 3 + 4 + 2,
    };

    let mesh = Mesh::new(
      BufferType::Static,
      vertices.to_vec(),
      Some(indices.to_vec()),
      descriptor,
    );

    let shader = ShaderProgram::new(
      "../assets/materials/planks/shaders/planks.vert",
      "../assets/materials/planks/shaders/planks.frag",
    );

    let diffuse = Sampler2D::<Diffuse>::new(
      "../assets/materials/planks/textures/diffuse.png",
      gl::RGBA,
      gl::UNSIGNED_BYTE
    );
    
    let specular = Sampler2D::<Specular>::new(
      "../assets/materials/planks/textures/specular.png",
      gl::RGBA,
      gl::UNSIGNED_BYTE
    );

    let u_data: [f32; 3] = [
      0.42,
      1.2,
      0.3,
    ];

    let ubo = UniformBufferObject::new((u_data.len() * std::mem::size_of::<f32>()) as isize);
    // ubo.set_data(0, &u_data);

    let mut material = Material::new(shader, app.info.texture_unit_count() as usize, Some(ubo));
    material.add_sampler(diffuse);
    material.add_sampler(specular);

    // material.set_ubo_data();

    let game_object = GameObject::new(mesh, material).with_transform(Transform::identity());

    let light = PointLight::new(
      Transform::identity(),
      cgmath::vec3(1.0 ,1.0 , 1.0),
      0.95,
      (0.42, 1.2, 0.3)
    );

    let ground_shader = ShaderProgram::new(
      "../shaders/ground.vert",
      "../shaders/ground.frag",
    );

    let ground_height_map = Sampler2D::<Topography>::new(
      "../assets/textures/perlin noise.png",
      gl::RGBA,
      gl::UNSIGNED_BYTE
    );

    let ground_texture = Sampler2D::<Diffuse>::new(
      "../assets/textures/bricks texture.jpg",
      gl::RGBA,
      gl::UNSIGNED_BYTE
    );

    let u_height_scale: [f32; 1] = [0.05];

    let ground_ubo = UniformBufferObject::new((u_height_scale.len() * std::mem::size_of::<f32>()) as isize);
    
    let mut ground_material = Material::new(ground_shader, app.info.texture_unit_count() as usize, Some(ground_ubo));
    ground_material.add_sampler(ground_height_map);
    ground_material.add_sampler(ground_texture);
    ground_material.shader.create_uniform("uHeightScale");

    let ground = Ground::new(64, 64, ground_material).with_transform(
      Transform {
        translation: cgmath::vec3(0.0, 0.2, 0.0),
        rotation: cgmath::Quaternion::one(),
        scale: cgmath::vec3(1.0, 1.0, 1.0),
      }
    );

    let camera = Camera::new(
      // 1280,
      // 720,
      1920,
      1080,
      (1280 / 720) as f32,
      cgmath::point3(0.0, 0.0, 3.0),
      45.0,
      0.1,
      100.0,
      0.4,
      100.0,
    );

    app.add_game_object(game_object);
    app.add_pointlight(light);
    app.add_ground(ground);
    app.add_camera(camera);

    app.run();
  }
}
