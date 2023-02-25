use crate::config::{Config, APP_NAME};
use iced::widget::{button, container, text};
use iced::{application, color};

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    cfg: Config,
}

impl Default for Theme {
    fn default() -> Self {
        let cfg = confy::load(APP_NAME, None).unwrap_or_default();
        Self { cfg }
    }
}

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: color!(0x282828),
            text_color: color!(self.cfg.text_color),
        }
    }
}

impl text::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: Self::Style) -> text::Appearance {
        text::Appearance::default()
    }
}

impl container::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance::default()
    }
}

impl button::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance::default()
    }
}
