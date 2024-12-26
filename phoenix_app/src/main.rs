mod config;
use glfw::Context;
use gl;

use crate::config::*;

fn fail_on_errors(error: glfw::Error, description: String) {
    eprintln!("GLFW Error ({}): {}", error, description);
}

pub mod ui {
    pub mod themes;
    pub mod hud;
}

pub enum PhoenixRunningMode {
    Edit,
    Game,
    Debug,
}

pub struct PhoenixGameEngineApplication {
// App
    window: glfw::PWindow,
    receiver: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    width: u32,
    height: u32,
    running_mode: PhoenixRunningMode,
    static_game_objects: bool,                                  // Vec<StaticGameObject>
    dynamic_game_objects: bool,                                 // Vec<DynamicGameObject>
    editor_camera: bool,                                        // Camera (PerspectiveCamera)
    game_camera: bool,                                          // Camera (PerspectiveCamera)
    asset_loader: bool,                                         // Asset Loader

// UI
    egui_painter: egui_glfw::Painter,
    egui_context: egui::Context,

// Engine
    engine_config: EngineConfig,
}

fn main() {
    let config = config::EngineConfig::new("Phoenix Engine v0.1", "../assets/icons/icon.png", 1920, 1080);
    let mut glfw = glfw::init(fail_on_errors).expect("Failed to init GLFW");

    glfw.window_hint(glfw::WindowHint::Resizable(true));

    let (mut window, receiver) = match glfw
    .create_window(
        config.width,
        config.height,
        &config.title,
        glfw::WindowMode::Windowed,
    ) {
        Some(window) => window,
        None => panic!("Failed to create window!")
    };

    window.make_current();
    window.set_all_polling(true);

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let version = unsafe { std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8) };
    println!("OpenGL version: {}", version.to_str().unwrap());

    window.set_icon_from_pixels(vec![config.icon]);

    // Launch UI here
    // let mut ui;

    use crate::ui::hud::*;
    let mut hud = Hud::new();

    let mode = PhoenixRunningMode::Edit;

    while !window.should_close() {
        glfw.poll_events();
        for (time, event) in glfw::flush_messages(&receiver) {
            println!("{:?}", (time, event));
        }

        match /* self. */ mode {
            PhoenixRunningMode::Edit => {
                // Render ui
                // hud.draw();
                
                // Render game stuff
            },
            PhoenixRunningMode::Game => {
                // Render ui
                // hud.draw();
                
                // Render game stuff
            },
            PhoenixRunningMode::Debug => {
                // Render ui
                // hud.draw();
                
                // Render game stuff
            }
        }

        // Finalize frame: Optionally sync framerate for fixed framerates
        window.swap_buffers();
    }

    // clean_up();
}