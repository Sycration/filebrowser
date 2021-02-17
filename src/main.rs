#![forbid(clippy::missing_docs_in_private_items)]

use iced::{Column, Element, Row, Sandbox, Settings, Text, button, Button};
use root_dir::root_dir;
use std::path::PathBuf;
mod root_dir;
mod quit;

///entry point, void main, yadda yadda
fn main() -> iced::Result {
    App::run(Settings::default())
}

///All the things with a state in the app
///Right now it just contains the current path because the program is not done
///This documentation will be more advanced later
struct App {
    current_dir: PathBuf,
    up_dir_button: button::State,
    home_dir_button: button::State,
}

///The Default (initial) state of the program is the user's home dir. If the user does not have
///a home dir, start at the root dir. As this is a file browser, running it on a system with
///no filesystem would be mad.
///  
///Also contains the state of all the other UI elements.
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
            up_dir_button: button::State::default(),
            home_dir_button: button::State::default(),
        }
    }
}

///Messages from the UI to the state, reacting to changes as the user interacts with the app
#[derive(Debug, Clone)]
enum Message {
    ChDir(PathBuf),
    UpDir,
}

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
        match message {
            Message::ChDir(path) => {
                if path.exists() {
                    if path.is_file() {
                        quit::quit(path);
                    } else if path.is_dir() {
                        self.current_dir = path.to_owned();
                    }
                }
            }
            Message::UpDir => {
                if let Some(dir) = self.current_dir.parent() {
                    self.current_dir = dir.to_path_buf();
                }
            }
        }
    }

    fn view(&mut self) -> Element<'_, Message> {
        //part of the UI that shows where we are at the top of the screen
        let dir_view = Text::new(self.current_dir.to_str().unwrap());

        //row of quick options at the top, like a bookmarks bar
        let button_row = Row::new()
            .padding(10)
            .spacing(10)
            .push(
                Button::new(&mut self.up_dir_button, Text::new("go up"))
                            .on_press(Message::UpDir)
            )
            .push(
                Button::new(&mut self.home_dir_button, Text::new("home dir"))
                .on_press(Message::ChDir(dirs::home_dir().unwrap_or_else(|| {root_dir()})))
            );
        Column::new()
        .push(dir_view)
        .push(button_row)
        .into()
    }
}
