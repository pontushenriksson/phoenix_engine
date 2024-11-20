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

  engine.add_resource(PhoenixEngine::create_texture2D("./assets/textures/goofy.jpg"));
  engine.add_resource(PhoenixEngine::create_shader("./shaders/shader.vert", "./shaders/shader.frag"));

  let pyramid: [f32; 45] = [
    // Vertices                  Colors                      Texture coordinates
    -0.5,  0.0,  0.5,          0.83, 0.70, 0.44, 1.0,         0.0, 0.0,
    -0.5,  0.0, -0.5,          0.83, 0.70, 0.44, 1.0,         5.0, 0.0,
     0.5,  0.0, -0.5,          0.83, 0.70, 0.44, 1.0,         0.0, 0.0,
     0.5,  0.0,  0.5,          0.83, 0.70, 0.44, 1.0,         5.0, 0.0,
     0.0,  0.8,  0.0,          0.92, 0.86, 0.76, 1.0,         2.5, 5.0,
  ];

  let indices: [u32; 21] = [
    0, 1, 2,
    0, 2, 3,
    0, 1, 4,
    1, 2, 4,
    1, 2, 4,
    2, 3, 4,
    3, 0, 4,
  ];

  engine.new_static_object(pyramid.to_vec(), indices.to_vec(), 0, 0); // later engine.new_component(PhoenixEngine::create_component());
  engine.new_camera_3d(WINDOW_WIDTH, WINDOW_HEIGHT, PhoenixEngine::world_space_point_3d(0.0, 0.0, 0.0));

  engine.run(|| {
    println!("Running!");
  });
}
