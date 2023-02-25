use chrono::Local;
use clock::theme::Theme;
use core::default::Default;
use iced::widget::{Container, Text};
use iced::{executor, Application, Command, Element, Renderer, Settings, Subscription};

const FONT: &[u8] = include_bytes!("../fonts/Terminus.ttf");
fn main() -> iced::Result {
    App::run(Settings {
        default_font: Some(FONT),
        window: iced::window::Settings {
            size: (980, 256),
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

struct App {
    time: String,
}

#[derive(Debug)]
enum Message {
    Tick,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let date_time = Local::now();
        (
            Self {
                time: format!("{}", date_time.format("%H:%M:%S")),
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
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let label = Text::new(&self.time).size(240);
        Container::new(label).into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(1000)).map(|_| Message::Tick)
    }
}
