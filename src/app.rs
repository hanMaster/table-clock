use crate::config::{Config, APP_NAME};
use crate::theme::Theme;
use chrono::Local;
use iced::widget::{Button, Column, Container, Text};
use iced::{executor, Alignment, Application, Command, Element, Length, Renderer, Subscription};

pub struct App {
    time: String,
    page: Pages,
    cfg: Config,
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
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let config = confy::load(APP_NAME, None).unwrap_or_default();
        println!("{:#?}", config);
        let path =
            confy::get_configuration_file_path(APP_NAME, None).expect("Failed to get config path");
        println!("Config path: {:#?}", path);
        let date_time = Local::now();
        (
            Self {
                time: format!("{}", date_time.format("%H:%M:%S")),
                page: Pages::Main,
                cfg: config,
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
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        match self.page {
            Pages::Main => {
                let setup_button = Button::new(Text::new("Enter Setup")).on_press(Message::Setup);
                let label = Text::new(&self.time).size(self.cfg.font_size);
                let content = Column::new()
                    .align_items(Alignment::Start)
                    .push(setup_button)
                    .push(label);
                Container::new(content).into()
            }
            Pages::Setup => {
                let done_button = Button::new(Text::new("Done"))
                    .width(Length::Fixed(100.))
                    .on_press(Message::SetupDone);
                let label = Text::new("Setup all settings for display").size(24);
                let content = Column::new()
                    .align_items(Alignment::Start)
                    .push(done_button)
                    .push(label);
                Container::new(content).into()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(1000)).map(|_| Message::Tick)
    }
}
