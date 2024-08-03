use iced::Sandbox;

#[derive(Debug, Clone, Copy)]
pub enum Message {}

pub struct Rmusicplayer;

impl Sandbox for Rmusicplayer {
    type Message = Message;

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("My App")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<Self::Message> {
        "Hello World!".into()
    }
}
