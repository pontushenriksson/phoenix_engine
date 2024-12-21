use egui::Context as EguiContext;
use egui_glow::Painter as EguiPainter;
use glfw::Context as _;
use std::sync::Arc;

pub struct EguiIntegration {
    egui_context: EguiContext,
    painter: EguiPainter,
}

impl EguiIntegration {
    pub fn new(window: &mut glfw::Window) -> Self {
        let egui_context = egui::Context::default();
        let painter = unsafe {
            EguiPainter::new(
                Arc::new(egui_glow::painter::Context::from_loader_function(|s| window.get_proc_address(s))),
                "egui_painter",
                None,
                true,
            )
            .unwrap()
        };

        Self { egui_context, painter }
    }

    pub fn begin_frame(&mut self, window: &glfw::Window) {
        let raw_input = translate_glfw_input(window);
        self.egui_context.begin_pass(raw_input);
    }
    

    pub fn end_frame(&mut self, window: &mut glfw::Window) {
        let output = self.egui_context.end_pass();
        let shapes = output.shapes;
        let clipped_shapes = self.egui_context.tessellate(shapes, 1.0);

        let (width, height) = window.get_framebuffer_size();
        unsafe {
            gl::Viewport(0, 0, width, height);
        }

        self.painter.paint_and_update_textures(
            [width as u32, height as u32],
            1.0,
            &clipped_shapes,
            &output.textures_delta,
        );

        if !output.platform_output.copied_text.is_empty() {
            window.set_clipboard_string(&output.platform_output.copied_text);
        }
    }
}

fn translate_glfw_input(window: &glfw::Window) -> egui::RawInput {
    let (width, height) = window.get_framebuffer_size();
    let (scale_x, _) = window.get_content_scale();

    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::pos2(0.0, 0.0),
            egui::vec2(width as f32 / scale_x, height as f32 / scale_x),
        )),
        focused: true, // Assume the window is focused
        modifiers: egui::Modifiers::default(),
        events: vec![], // Populate this with input events as needed
        ..Default::default()
    }
}



fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, _events) = glfw
        .create_window(800, 600, "Game Engine with Egui", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");
    
    window.make_current();
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let mut egui_integration = EguiIntegration::new(&mut window);

    while !window.should_close() {
        glfw.poll_events();

        egui_integration.begin_frame(&window);

        egui::CentralPanel::default().show(&egui_integration.egui_context, |ui| {
            ui.label("Hello, egui!");
            if ui.button("Click me").clicked() {
                println!("Button clicked!");
            }
        });

        unsafe {
            gl::ClearColor(0.1, 0.2, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        egui_integration.end_frame(&mut window);
        window.swap_buffers();
    }
}
