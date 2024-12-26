pub struct Settings;

impl Settings {
    pub fn new() -> Self {
        Self
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");
        ui.label("Configure stuff here.");
    }
}