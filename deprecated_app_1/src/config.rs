use image::GenericImageView;

pub struct EngineConfig {
    pub title: String,
    pub icon: glfw::PixelImage,
    pub width: u32,
    pub height: u32,
}

impl EngineConfig {
    pub fn new(title: &str, icon_path: &str, width: u32, height: u32) -> EngineConfig {
        let icon = image::open(icon_path).unwrap();
        let (image_width, image_height) = icon.dimensions();
        let icon_pixels = rgba_u8_as_u32(icon.to_rgba8().into_raw());

        let glfw_icon = glfw::PixelImage {
            width: image_width as u32,
            height: image_height as u32,
            pixels: icon_pixels,
        };

        EngineConfig {
            title: title.to_owned(),
            icon: glfw_icon,
            width: width,
            height: height,
        }
    }
}

fn rgba_u8_as_u32(rgba_data: Vec<u8>) -> Vec<u32> {
    rgba_data.chunks(4).map(|rgba| {
        (rgba[0] as u32) << 24 | // Red
        (rgba[1] as u32) << 16 | // Green
        (rgba[2] as u32) << 8  | // Blue
        (rgba[3] as u32)         // Alpha
    }).collect()
}
