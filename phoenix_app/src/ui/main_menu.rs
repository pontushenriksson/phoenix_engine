pub struct MainMenu;

impl MainMenu {
    pub fn new() -> Self {
        Self
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        ui.heading("Phoenix Engine");
        // Pretend to show a logo (assets/icons/logo.png)
        ui.label("[Logo Here]");
        if ui.button("Go to Settings").clicked() {
            // Switch state
        }
        if ui.button("Go to HUD").clicked() {
            // Switch state
        }
    }
}