// src/config.rs (example)
pub struct EngineConfig {
    pub title: String,
    pub icon_path: String,
    pub width: u32,
    pub height: u32,
}

impl EngineConfig {
    pub fn new(title: &str, icon_path: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_owned(),
            icon_path: icon_path.to_owned(),
            width,
            height,
        }
    }
}