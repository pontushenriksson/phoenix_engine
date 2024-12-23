use eframe::egui;
use eframe::NativeOptions;
use image::ImageReader;
use std::path::Path;

struct MyApp {
    logo_texture: Option<egui::TextureHandle>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { logo_texture: None }
    }
}

impl MyApp {
    fn load_logo(&mut self, ctx: &egui::Context) {
        if self.logo_texture.is_none() {
            let image_path = Path::new("../assets/icons/icon.png");
            if let Ok(image_reader) = ImageReader::open(image_path) {
                if let Ok(image) = image_reader.decode() {
                    let size = [image.width() as usize, image.height() as usize];
                    let pixels = image.to_rgba8().as_flat_samples().as_slice().to_vec();
                    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
                    self.logo_texture = Some(ctx.load_texture("logo", color_image, Default::default()));
                } else {
                    eprintln!("Failed to decode image at {:?}", image_path);
                }
            } else {
                eprintln!("Failed to open image at {:?}", image_path);
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.load_logo(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(texture) = &self.logo_texture {
                ui.image(texture);
            } else {
                ui.label("Failed to load logo image.");
            }

            ui.add_space(10.0);
            ui.heading("Phoenix Engine");
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(egui::vec2(1920.0, 1080.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Phoenix Engine",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
