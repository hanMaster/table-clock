use crate::config::{Config, APP_NAME};
use iced::widget::{button, container, text};
use iced::{application, color};
use iced_aw::native::color_picker;
use iced_aw::native::color_picker::Appearance;

#[derive(Debug, Clone, Copy)]
pub struct MyTheme {
    cfg: Config,
}

impl Default for MyTheme {
    fn default() -> Self {
        let cfg = confy::load(APP_NAME, None).unwrap_or_default();
        Self { cfg }
    }
}

impl application::StyleSheet for MyTheme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: color!(0x282828),
            text_color: color!(self.cfg.text_color),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Text {
    FromConfig,
    #[default]
    Setup,
}

impl text::StyleSheet for MyTheme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        match style {
            Text::FromConfig => text::Appearance {
                color: color!(self.cfg.text_color).into(),
            },
            Text::Setup => text::Appearance {
                color: color!(0xff, 0xff, 0xff).into(),
            },
        }
    }
}

impl container::StyleSheet for MyTheme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance::default()
    }
}

impl color_picker::StyleSheet for MyTheme {
    type Style = ();

    fn active(&self, _style: Self::Style) -> Appearance {
        Appearance {
            background: color!(0x151515).into(),
            border_radius: 15.0,
            border_width: 1.0,
            border_color: color!(0x45, 0x85, 0x88),
            bar_border_radius: 5.0,
            bar_border_width: 1.0,
            bar_border_color: color!(0x45, 0x85, 0x88),
        }
    }

    fn selected(&self, style: Self::Style) -> Appearance {
        Appearance {
            ..self.active(style)
        }
    }

    fn hovered(&self, style: Self::Style) -> Appearance {
        Appearance {
            ..self.active(style)
        }
    }

    fn focused(&self, style: Self::Style) -> Appearance {
        Appearance {
            border_color: color!(0x030303),
            bar_border_color: color!(0x030303),
            ..self.active(style)
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Button {
    #[default]
    Primary,
    Text,
}

impl button::StyleSheet for MyTheme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Primary => button::Appearance {
                background: color!(0x45, 0x85, 0x88).into(),
                border_radius: 4.0,
                border_width: 1.0,
                border_color: color!(0x45, 0x85, 0x88),
                text_color: color!(0xff, 0xff, 0xff),
                ..Default::default()
            },
            Button::Text => button::Appearance {
                text_color: color!(0xff, 0xff, 0xff),
                ..Default::default()
            },
        }
    }
}
