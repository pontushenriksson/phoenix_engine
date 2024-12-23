pub mod core;

pub mod assets {
  pub mod manager;
  pub mod loader;
}

pub mod debugger {
  pub mod debugger;
  pub mod logger;
}

pub mod ecs {
  pub mod manager;
}

pub mod events {
  pub mod manager;
}

pub mod graphics {
  pub mod camera;
  pub mod data;
  pub mod mesh;
  pub mod object;
  pub mod renderer;
  pub mod shader;
  pub mod texture;
}

pub mod layers {
  pub mod layer;
  pub mod debug;
  pub mod game;
  pub mod ui;
}

pub mod scenes {
  pub mod loader;
  pub mod manager;
  pub mod scene;
}

pub mod window {
  pub mod window;
}

#[cfg(test)]
mod tests {
  use tokio;
  // use super::*;
  use crate::{assets::loader::RawVertexData, core, graphics::{camera::{Camera, PerspectiveCamera}, data::RenderData, mesh::StaticMesh, object::{StaticGameObject, Transform}, shader::ShaderProgram}, layers::game::GameLayer, scenes::scene::Scene};
  // use crate::debugger;
  // use crate::graphics::*;

  #[tokio::test]
  async fn phoenix_core() {
    let mut core = core::PhoenixCore::new(
      800,
      800,
      "phoenix_core",
      "../assets/icons/icon.png"
    ).unwrap();

    // Load assets, schedule loading of assets
    // Create game objects and define entities in the ECS
    // Push layers
    // Scenes

    let vertices: [f32; 216] = [
      // Positions        // Normals         // Colors
      // Front face
      -0.5, -0.5,  0.5,    0.0,  0.0,  1.0,   1.0, 0.0, 0.0, // Bottom-left
       0.5, -0.5,  0.5,    0.0,  0.0,  1.0,   0.0, 1.0, 0.0, // Bottom-right
       0.5,  0.5,  0.5,    0.0,  0.0,  1.0,   0.0, 0.0, 1.0, // Top-right
      -0.5,  0.5,  0.5,    0.0,  0.0,  1.0,   1.0, 1.0, 0.0, // Top-left
      
      // Back face
      -0.5, -0.5, -0.5,    0.0,  0.0, -1.0,   1.0, 0.0, 1.0, // Bottom-left
       0.5, -0.5, -0.5,    0.0,  0.0, -1.0,   0.0, 1.0, 1.0, // Bottom-right
       0.5,  0.5, -0.5,    0.0,  0.0, -1.0,   1.0, 1.0, 1.0, // Top-right
      -0.5,  0.5, -0.5,    0.0,  0.0, -1.0,   0.5, 0.5, 0.5, // Top-left
      
      // Left face
      -0.5, -0.5, -0.5,   -1.0,  0.0,  0.0,   1.0, 0.5, 0.0, // Bottom-left
      -0.5, -0.5,  0.5,   -1.0,  0.0,  0.0,   0.5, 1.0, 0.5, // Bottom-right
      -0.5,  0.5,  0.5,   -1.0,  0.0,  0.0,   0.0, 0.5, 1.0, // Top-right
      -0.5,  0.5, -0.5,   -1.0,  0.0,  0.0,   0.5, 0.5, 0.5, // Top-left
      
      // Right face
       0.5, -0.5, -0.5,    1.0,  0.0,  0.0,   0.5, 0.0, 1.0, // Bottom-left
       0.5, -0.5,  0.5,    1.0,  0.0,  0.0,   0.5, 0.5, 1.0, // Bottom-right
       0.5,  0.5,  0.5,    1.0,  0.0,  0.0,   0.0, 1.0, 0.5, // Top-right
       0.5,  0.5, -0.5,    1.0,  0.0,  0.0,   1.0, 1.0, 0.0, // Top-left
      
      // Top face
      -0.5,  0.5,  0.5,    0.0,  1.0,  0.0,   0.5, 1.0, 1.0, // Bottom-left
       0.5,  0.5,  0.5,    0.0,  1.0,  0.0,   0.0, 0.5, 0.5, // Bottom-right
       0.5,  0.5, -0.5,    0.0,  1.0,  0.0,   1.0, 0.5, 0.5, // Top-right
      -0.5,  0.5, -0.5,    0.0,  1.0,  0.0,   1.0, 1.0, 0.5, // Top-left
      
      // Bottom face
      -0.5, -0.5,  0.5,    0.0, -1.0,  0.0,   0.0, 0.0, 1.0, // Bottom-left
       0.5, -0.5,  0.5,    0.0, -1.0,  0.0,   1.0, 0.5, 1.0, // Bottom-right
       0.5, -0.5, -0.5,    0.0, -1.0,  0.0,   1.0, 0.0, 0.5, // Top-right
      -0.5, -0.5, -0.5,    0.0, -1.0,  0.0,   0.5, 0.5, 0.5, // Top-left
    ];

    let indices: [u32; 36] = [
      // Front face
      0, 1, 2, 0, 2, 3,
      // Back face
      4, 5, 6, 4, 6, 7,
      // Left face
      8, 9, 10, 8, 10, 11,
      // Right face
      12, 13, 14, 12, 14, 15,
      // Top face
      16, 17, 18, 16, 18, 19,
      // Bottom face
      20, 21, 22, 20, 22, 23,
    ];

    let vertex_data = RawVertexData {
      data: vertices.to_vec(),
      stride: 9
    };

    let mesh = StaticMesh::new(vertex_data, Some(indices.to_vec()));

    let shader = ShaderProgram::new("../shaders/no_textures.vert", "../shaders/no_textures.frag");
    
    let camera = PerspectiveCamera::new(
      cgmath::point3(0.0, 0.0, 0.0),
      cgmath::point3(0.0, 0.0, 0.0,),
      cgmath::vec3(0.0, 1.0, 0.0), (800 / 800) as f32, 45.0, 0.1, 100.0
    );

    core.game_layer.push_scene(Scene::new(Box::new(camera)));
    
    // Register asset
    core.add_static_game_object(StaticGameObject::from(mesh, Transform::none())); // Change later
    
    core.add_shader_program(shader);

    core.run();
  }
}
