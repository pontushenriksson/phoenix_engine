// src/main.rs (snippet)
use crate::config::EngineConfig;

fn main() {
    let config = EngineConfig::new("Phoenix App", "../assets/icons/icon.png", 1280, 720);
    let mut glfw = glfw::init(fail_on_errors).expect("Failed to initialize GLFW");

    let (mut window, events) = glfw
        .create_window(config.width, config.height, &config.title, WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    // Then launch [`MainMenu`](src/ui/main_menu.rs) or other UI
}