mod config;
use glfw::Context;
use gl;

fn fail_on_errors(error: glfw::Error, description: String) {
    eprintln!("GLFW Error ({}): {}", error, description);
}

pub mod ui {
    pub mod main_menu;
    pub mod settings;
    pub mod hud;
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

    while !window.should_close() {
        glfw.poll_events();
        for (time, event) in glfw::flush_messages(&receiver) {
            println!("{:?}", (time, event));
        }
        window.swap_buffers();
    }
}
