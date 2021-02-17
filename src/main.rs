use iced::{Column, Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Default)]
struct App {

}

#[derive(Debug, Clone, Copy)]
enum Message {
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("File Browser")
    }

    fn update(&mut self, message: Message) {
        match message {
           
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .into()
    }
}