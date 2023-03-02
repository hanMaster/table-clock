use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Config {
    pub font_size: f32,
    pub text_color: [f32; 4],
}

impl Default for Config {
    fn default() -> Self {
        Self {
            font_size: 240.,
            text_color: [0.9, 0.88, 0.67, 1.],
        }
    }
}

pub const APP_NAME: &'static str = "table-clock";

pub const FONT: &[u8] = include_bytes!("../fonts/Terminus.ttf");
