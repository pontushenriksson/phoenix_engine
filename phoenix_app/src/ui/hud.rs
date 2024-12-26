pub struct Hud;

impl Hud {
    pub fn new() -> Self {
        Self
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        ui.heading("Game HUD");
        ui.label("HUD elements here.");
    }
}