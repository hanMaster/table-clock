use crate::config::{Config, APP_NAME};
use crate::theme;
use crate::theme::MyTheme;
use chrono::Local;
use iced::widget::{row, Button, Column, Container, Row, Text};
use iced::{executor, Alignment, Application, Color, Command, Element, Renderer, Subscription};
use iced_aw::ColorPicker;

pub struct App {
    time: String,
    page: Pages,
    cfg: Config,
    show_picker: bool,
}

enum Pages {
    Main,
    Setup,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    Setup,
    SetupDone,
    IncreaseFont,
    DecreaseFont,
    ChooseColor,
    SubmitColor(Color),
    CancelColor,
    SetDefault,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = MyTheme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let config = confy::load(APP_NAME, None).unwrap_or_default();
        // let path =
        //     confy::get_configuration_file_path(APP_NAME, None).expect("Failed to get config path");
        // println!("Config path: {:#?}", path);
        let date_time = Local::now();
        (
            Self {
                time: format!("{}", date_time.format("%H:%M:%S")),
                page: Pages::Main,
                cfg: config,
                show_picker: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Настольные часы")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Tick => {
                self.time = format!("{}", Local::now().format("%H:%M:%S"));
            }
            Message::Setup => {
                self.page = Pages::Setup;
            }
            Message::SetupDone => {
                self.page = Pages::Main;
                confy::store(APP_NAME, None, self.cfg).expect("Failed to save config");
            }
            Message::IncreaseFont => {
                self.cfg.font_size += 4.;
            }
            Message::DecreaseFont => {
                self.cfg.font_size -= 4.;
            }
            Message::ChooseColor => {
                self.show_picker = true;
            }
            Message::SubmitColor(color) => {
                self.cfg.text_color = color.into_linear();
                self.show_picker = false;
            }
            Message::CancelColor => {
                self.show_picker = false;
            }
            Message::SetDefault => {
                self.cfg.text_color = [0., 1., 0., 1.];
                self.cfg.font_size = 240.;
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        match self.page {
            Pages::Main => {
                let setup_button = Button::new(Text::new("*"))
                    .style(theme::Button::Text)
                    .on_press(Message::Setup);
                let label = Text::new(&self.time)
                    .style(theme::Text::FromConfig)
                    .size(self.cfg.font_size);
                let content = Column::new()
                    .align_items(Alignment::Start)
                    .push(setup_button)
                    .push(label);
                Container::new(content).into()
            }
            Pages::Setup => {
                let done_button = Button::new(Text::new("Сохранить"))
                    .style(theme::Button::Text)
                    .on_press(Message::SetupDone);
                let inc_btn = Button::new(Text::new(" + "))
                    .style(theme::Button::Primary)
                    .on_press(Message::IncreaseFont);
                let dec_btn = Button::new(Text::new(" - "))
                    .style(theme::Button::Primary)
                    .on_press(Message::DecreaseFont);
                let label = Text::new(format!("Размер шрифта: {}", self.cfg.font_size)).size(24);
                let font_row = row![label, inc_btn, dec_btn]
                    .spacing(10)
                    .padding(10)
                    .align_items(Alignment::Center);

                let color_button = Button::new(Text::new("Выбрать цвет текста"))
                    .style(theme::Button::Primary)
                    .on_press(Message::ChooseColor);

                let datepicker = ColorPicker::new(
                    self.show_picker,
                    Color::from(self.cfg.text_color),
                    color_button,
                    Message::CancelColor,
                    Message::SubmitColor,
                );
                let color_row = Row::new()
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .padding(10)
                    .push(Text::new("Образец текста").style(theme::Text::FromConfig))
                    .push(datepicker);
                let default_btn = Button::new(Text::new("Сброс настроек"))
                    .style(theme::Button::Primary)
                    .on_press(Message::SetDefault);
                let default_row = Row::new()
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .padding(10)
                    .push(default_btn);
                let content = Column::new()
                    .align_items(Alignment::Start)
                    .push(done_button)
                    .push(font_row)
                    .push(color_row)
                    .push(default_row);
                Container::new(content).into()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(1000)).map(|_| Message::Tick)
    }
}
