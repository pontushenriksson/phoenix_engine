use phoenix_core::core::*;
use phoenix_core::core::bindings::*;
use phoenix_core::ecs::components::*;
use phoenix_core::graphics::shaders::*;
use phoenix_core::debugger::debugger::*;

mod scripts {
    pub mod object;
}

use scripts::*;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

fn main() {
  let mut engine = PhoenixEngine::new(WINDOW_WIDTH, WINDOW_HEIGHT, "Test Game", "./assets/icons/icon.png", DebuggerRunningMode::Accumulate(DebuggerOutputMode::File));

  engine.add_resource(PhoenixEngine::create_texture2D("./assets/textures/goofy.jpg", TextureType::Diffuse)); // index 0
  
  engine.add_resource(PhoenixEngine::create_shader("./shaders/default.vert", "./shaders/default.frag")); // index 0
  engine.add_resource(PhoenixEngine::create_shader("./shaders/light.vert", "./shaders/light.frag"));     // index 1

  let pyramid: [f32; 192] = [
  // Vertices                Colors                        Texture        Normals
    -0.5,  0.0,  0.5,        0.83, 0.70, 0.44, 1.0,        0.0, 0.0,       0.0, -1.0,  0.0, // Bottom side
    -0.5,  0.0, -0.5,        0.83, 0.70, 0.44, 1.0,        0.0, 5.0,       0.0, -1.0,  0.0,
     0.5,  0.0, -0.5,        0.83, 0.70, 0.44, 1.0,        5.0, 5.0,       0.0, -1.0,  0.0,
     0.5,  0.0,  0.5,        0.83, 0.70, 0.44, 1.0,        5.0, 0.0,       0.0, -1.0,  0.0,

    -0.5,  0.0,  0.5,        0.83, 0.70, 0.44, 1.0,        0.0, 0.0,      -0.8,  0.5,  0.0, // Left side
    -0.5,  0.0, -0.5,        0.83, 0.70, 0.44, 1.0,        5.0, 0.0,      -0.8,  0.5,  0.0,
     0.0,  0.8,  0.0,        0.92, 0.86, 0.76, 1.0,        2.5, 5.0,      -0.8,  0.5,  0.0,

    -0.5,  0.0, -0.5,        0.83, 0.70, 0.44, 1.0,        5.0, 0.0,       0.0,  0.5, -0.8, // Non-facing side
     0.5,  0.0, -0.5,        0.83, 0.70, 0.44, 1.0,        0.0, 0.0,       0.0,  0.5, -0.8,
     0.0,  0.8,  0.0,        0.92, 0.86, 0.76, 1.0,        2.5, 5.0,       0.0,  0.5, -0.8,

     0.5,  0.0, -0.5,        0.83, 0.70, 0.44, 1.0,        0.0, 0.0,       0.8,  0.5,  0.0, // Right side
     0.5,  0.0,  0.5,        0.83, 0.70, 0.44, 1.0,        5.0, 0.0,       0.8,  0.5,  0.0,
     0.0,  0.8,  0.0,        0.92, 0.86, 0.76, 1.0,        2.5, 5.0,       0.8,  0.5,  0.0,

     0.5,  0.0,  0.5,        0.83, 0.70, 0.44, 1.0,        5.0, 0.0,       0.0,  0.5,  0.8, // Facing side
    -0.5,  0.0,  0.5,        0.83, 0.70, 0.44, 1.0,        0.0, 0.0,       0.0,  0.5,  0.8,
     0.0,  0.8,  0.0,        0.92, 0.86, 0.76, 1.0,        2.5, 5.0,       0.0,  0.5,  0.8,
  ];

  let pyramid_indices: [u32; 18] = [
    0, 1, 2,    // Bottom
    0, 2, 3,
    4, 6, 5,    // Left
    7, 9, 8,    // Non-facing
    10, 12, 11, // Right
    13, 15, 14, // Facing
  ];

  let light: [f32; 72] = [
    // Vertices                // Place holder data since VertexAttribute isn't dynamic yet
    -0.1, -0.1,  0.1,          1.00, 1.00, 1.00, 1.0,         0.0, 0.0,
    -0.1, -0.1, -0.1,          1.00, 1.00, 1.00, 1.0,         0.0, 0.0,
     0.1, -0.1, -0.1,          1.00, 1.00, 1.00, 1.0,         0.0, 0.0,
     0.1, -0.1,  0.1,          1.00, 1.00, 1.00, 1.0,         0.0, 0.0,
    -0.1,  0.1,  0.1,          1.00, 1.00, 1.00, 1.0,         0.0, 0.0,
    -0.1,  0.1, -0.1,          1.00, 1.00, 1.00, 1.0,         0.0, 0.0,
     0.1,  0.1, -0.1,          1.00, 1.00, 1.00, 1.0,         0.0, 0.0,
     0.1,  0.1,  0.1,          1.00, 1.00, 1.00, 1.0,         0.0, 0.0,
  ];

  let light_indices: [u32; 36] = [
    0, 1, 2,
    0, 2, 3,
    0, 4, 7,
    0, 7, 3,
    3, 7, 6,
    3, 6, 2,
    2, 6, 5,
    2, 5, 1,
    1, 5, 4,
    1, 4, 0,
    4, 5, 6,
    4, 6, 7,
  ];

  // later engine.new_component(PhoenixEngine::create_component());

  engine.new_static_object(
    pyramid.to_vec(),
    pyramid_indices.to_vec(), 
    0,
    0,
    PhoenixEngine::world_space_vector_3d(0.0, 0.0, 0.0),
    PhoenixEngine::id_matrix()
  );
  
  engine.new_static_object(
    light.to_vec(),
    light_indices.to_vec(),
    -1,
    1,
    PhoenixEngine::world_space_vector_3d(0.5, 0.5, 0.5),
    PhoenixEngine::id_matrix()
  );

  engine.new_camera_3d(WINDOW_WIDTH, WINDOW_HEIGHT, PhoenixEngine::world_space_point_3d(0.0, 0.0, 2.0));

  engine.run(|| {
    println!("Running!");
  });
}
