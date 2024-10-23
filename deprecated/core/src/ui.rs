// ui.rs
use egui_glfw_gl as egui_backend;

pub struct UI {
  egui_context: egui::Context,
  egui_renderer: egui_backend::Renderer, // or any compatible backend
}

impl UI {
  pub fn new() -> Self {
    // Initialize Egui context and renderer
    Self {
      egui_context: egui::Context::new(),
      egui_renderer: egui_backend::Renderer::new(),
    }
  }

  pub fn render(&mut self) {
    let output = self.egui_context.run(..., |ctx| {
      egui::CentralPanel::default().show(ctx, |ui| {
        ui.label("Hello, world!");
        if ui.button("Quit").clicked() {
          // Handle button clicks
        }
      });
    });

    // Pass output to the renderer
    self.egui_renderer.render(...);
  }

  pub fn handle_input(&mut self, event: &glfw::WindowEvent) {
    // Handle input events for Egui
  }
}
