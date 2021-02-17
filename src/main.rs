#![forbid(clippy::missing_docs_in_private_items)]

use std::{path::{PathBuf}};
use iced::{Column, Element, Sandbox, Settings};
mod root_dir;

///entry point, void main, yadda yadda
fn main() -> iced::Result {
    App::run(Settings::default())
}


///All the things with a state in the app
///Right now it just contains the current path because the program is not done
///This documentation will be more advanced later
struct App {
    current_dir: PathBuf,
}

///The Default (initial) state of the program is the user's home dir. If the user does not have
///a home dir, start at the root dir. As this is a file browser, running it on a system with
///no filesystem would be mad.
impl Default for App {
    fn default() -> Self {
        Self {
            current_dir: { 
                if let Some(home_dir) = dirs::home_dir() {
                    home_dir
                } else {
                    root_dir::root_dir()
                }
            },
        }
    }
}

///One day, the application will be able to change. For now, there is no program so it
///will never change.
#[derive(Debug, Clone, Copy)]
enum Message {}

///Iced library magic
impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("File Browser")
    }

    fn update(&mut self, message: Message) {
        match message {}
    }

    fn view(&mut self) -> Element<'_, Message> {
        Column::new().into()
    }
}
