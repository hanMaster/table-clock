use clock::app::App;
use clock::config::{Config, APP_NAME, FONT};
use core::default::Default;
use iced::window::Position;
use iced::{Application, Settings};

fn main() -> iced::Result {
    let config: Config = confy::load(APP_NAME, None).unwrap_or_default();
    let font_size = config.font_size;
    let width = (font_size * 4.1) as u32;
    let height = (font_size + 30.) as u32;
    App::run(Settings {
        default_font: Some(FONT),
        window: iced::window::Settings {
            size: (width, height),
            position: Position::Default,
            visible: true,
            resizable: true,
            decorations: true,
            always_on_top: true,
            ..Default::default()
        },
        ..Default::default()
    })?;
    Ok(())
}
