use phoenix_core::core::*;
use phoenix_core::core::bindings::*;
use phoenix_core::ecs::components::*;
use phoenix_core::graphics::shaders::*;

mod scripts {
    pub mod object;
}

use scripts::*;

fn main() {
  let mut engine = PhoenixEngine::new(800, 800, "Test Game", "./assets/icons/icon.png");

  engine.add_resource(PhoenixEngine::create_texture2D("./assets/textures/goofy.jpg"));
  engine.add_resource(PhoenixEngine::create_shader("./shaders/shader.vert", "./shaders/shader.frag"));

  let data: [f32; 36] = [
    // Vertices                  Colors                      Texture coordinates
    0.5,  0.5,  0.0,          1.0, 0.0, 0.0, 1.0,         1.0, 1.0, // Top right
    0.5, -0.5,  0.0,          0.0, 1.0, 0.0, 1.0,         1.0, 0.0, // Bottom right
   -0.5, -0.5,  0.0,          0.0, 0.0, 1.0, 1.0,         0.0, 0.0, // Bottom left
   -0.5,  0.5,  0.0,          1.0, 1.0, 0.0, 1.0,         0.0, 1.0, // Top left
  ];

  let indices: [u32; 6] = [
    0, 1, 3,
    1, 2, 3,
  ];

  engine.new_static_object(data.to_vec(), indices.to_vec(), 0, 0); // later engine.new_component(PhoenixEngine::create_component());
  

  engine.run(|| {
    /*
    object::update();
    object::render();
    */

    println!("Running!");
  });
}
