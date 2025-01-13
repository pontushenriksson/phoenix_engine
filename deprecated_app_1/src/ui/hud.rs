pub struct Hud;

impl Hud {
    pub fn new() -> Self {
        Self
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        ui.heading("Game HUD");
        ui.label("HUD elements here.");

        egui::TopBottomPanel::top("navbar").show_inside(ui, |ui| {
            ui.horizontal_centered(|ui| {
                if ui.button("Button 1").clicked() {
                    // Handle button 1
                }
                if ui.button("Button 2").clicked() {
                    // Handle button 2
                }
                if ui.button("Button 3").clicked() {
                    // Handle button 3
                }
                if ui.button("Button 4").clicked() {
                    // Handle button 4
                }
            });
        });
    }
}