pub struct Settings;

impl Settings {
    pub fn new() -> Self {
        Self
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");
        ui.label("Configure stuff here.");

        // This should only be 3 different themes that oculd be used to color the editor in different ways. This should be changed in a dropdown form the hud and not have its own page.
    }
}