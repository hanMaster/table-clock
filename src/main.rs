use clock::app::App;
use core::default::Default;
use iced::window::Position;
use iced::{Application, Settings};

const FONT: &[u8] = include_bytes!("../fonts/Terminus.ttf");
fn main() -> iced::Result {
    App::run(Settings {
        default_font: Some(FONT),
        window: iced::window::Settings {
            size: (980, 256),
            position: Position::Specific(0, 900),
            visible: true,
            resizable: false,
            decorations: true,
            always_on_top: true,
            ..Default::default()
        },
        ..Default::default()
    })?;
    Ok(())
}
