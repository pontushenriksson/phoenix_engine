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
  use crate::{assets::loader::RawVertexData, core, graphics::{data::{RenderData, VertexArrayObject}, mesh::Mesh}};
  // use crate::debugger;
  // use crate::graphics::*;

  #[tokio::test]
  async fn phoenix_core() {
    let core = core::PhoenixCore::new(
      800,
      800,
      "phoenix_core",
      "C:/dev/phoenix/phoenix_engine/assets/icons/icon.png"
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

    /* let mesh = Mesh {
      vertices: vertex_data,
      indices: Some(indices.to_vec()),
      render_data: 
    }; */
    
    core.run();
  }
}
