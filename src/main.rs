#![forbid(clippy::missing_docs_in_private_items)]

use files::button_list_gen;
use iced::{
    button, scrollable, Button, Column, Element, Length, Row, Sandbox, Scrollable, Settings, Space,
    Text,
};
use root_dir::root_dir;
use std::path::PathBuf;
use string::correct_string;
mod files;
mod quit;
mod root_dir;
mod string;

///entry point, void main, yadda yadda
pub fn main() -> iced::Result {
    App::run(Settings::default())
}

///All the things with a state in the app.
///This includes all the buttons and the scrolling panel
struct App {
    current_dir: PathBuf,
    up_dir_button: button::State,
    home_dir_button: button::State,
    list_of_files: Vec<PathBuf>,
    list_of_dirs: Vec<PathBuf>,
    file_buttons: Vec<button::State>,
    dir_buttons: Vec<button::State>,
    scroll: scrollable::State,
    chosen_file: bool,
    confirm_button: button::State,
    cancel_button: button::State,
    selected_file: PathBuf,
}

///The Default (initial) state of the program is the user's home dir. If the user does not have
///a home dir, start at the root dir. As this is a file browser, running it on a system with
///no filesystem would be mad.
///  
///Also contains the state of all the other UI elements.
impl Default for App {
    fn default() -> Self {
        let current_dir = {
            if let Some(home_dir) = dirs::home_dir() {
                home_dir
            } else {
                root_dir::root_dir()
            }
        };
        let mut file_states = Vec::new();
        let mut dir_states = Vec::new();
        let mut list = files::get_list(&current_dir);
        list.1.sort();
        list.0.sort();
        button_list_gen(&mut file_states, &list.0);
        button_list_gen(&mut dir_states, &list.1);
        Self {
            current_dir: current_dir.clone(),
            up_dir_button: button::State::default(),
            home_dir_button: button::State::default(),
            list_of_files: list.0,
            list_of_dirs: list.1,
            file_buttons: file_states,
            dir_buttons: dir_states,
            scroll: scrollable::State::new(),
            chosen_file: false,
            confirm_button: button::State::default(),
            cancel_button: button::State::default(),
            selected_file: PathBuf::new(),
        }
    }
}

///Messages from the UI to the state, reacting to changes as the user interacts with the app
#[derive(Debug, Clone)]
enum Message {
    ChDir(Option<PathBuf>),
    ChConfirm(bool),
    Quit(PathBuf),
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
                if let Some(path) = path {
                    if path.exists() {
                        if path.is_file() {
                            self.chosen_file = true;
                            self.selected_file = path;
                        } else if path.is_dir() {
                            self.list_of_dirs.clear();
                            self.list_of_files.clear();
                            self.chosen_file = false;
                            self.current_dir = path.to_owned();
                            let mut list = files::get_list(&self.current_dir);
                            list.1.sort();
                            list.0.sort();
                            button_list_gen(&mut self.file_buttons, &list.0);
                            button_list_gen(&mut self.dir_buttons, &list.1);
                            self.list_of_dirs = list.1;
                            self.list_of_files = list.0;
                        }
                    }
                }
            }
            Message::ChConfirm(state) => {
                self.chosen_file = state;
            }
            Message::Quit(path) => {
                quit::quit(path);
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
                Button::new(&mut self.up_dir_button, Text::new("go up")).on_press({
                    if let Some(dir) = self.current_dir.parent() {
                        Message::ChDir(Some(dir.to_path_buf()))
                    } else {
                        Message::ChDir(None)
                    }
                }),
            )
            .push(
                Button::new(&mut self.home_dir_button, Text::new("home dir")).on_press(
                    Message::ChDir(Some(dirs::home_dir().unwrap_or_else(|| root_dir()))),
                ),
            );

        //Makes sure that the user does not accidentally select a file they do not mean to
        let confirm_bar = {
            match self.chosen_file {
                true => Row::new()
                    .push(Text::new(self.selected_file.to_str().unwrap()))
                    .push(Space::new(Length::Fill, Length::Units(0)))
                    .push(
                        Button::new(&mut self.cancel_button, Text::new("Cancel"))
                            .on_press(Message::ChConfirm(false))
                            .height(Length::Fill),
                    )
                    .push(
                        Button::new(&mut self.confirm_button, Text::new("Confirm"))
                            .on_press(Message::Quit(self.selected_file.clone()))
                            .height(Length::Fill),
                    )
                    .height(Length::Units(40)),
                false => Row::new()
                    .push(Space::new(Length::Fill, Length::Units(0)))
                    .height(Length::Units(0)),
            }
        };

        let files = self
            .file_buttons
            .iter_mut()
            .zip(self.list_of_files.iter())
            .fold(
                Column::new().width(Length::Fill).spacing(5),
                |scroll, (a, b)| {
                    scroll.push(
                        Button::new(a, Text::new( b.file_name().unwrap().to_str().unwrap().to_string()))
                            .width(Length::Fill)
                            .on_press(Message::ChDir(Some(b.to_path_buf()))),
                    )
                },
            );

        let dirs = self
            .dir_buttons
            .iter_mut()
            .zip(self.list_of_dirs.iter())
            .fold(
                Column::new().width(Length::Fill).spacing(5),
                |scroll, (a, b)| {
                    scroll.push(
                        Button::new(
                            a,
                            Text::new({
                                let mut x = b.file_name().unwrap().to_str().unwrap().to_string();
                                x.push('/');
                                x
                            }),
                        )
                        .width(Length::Fill)
                        .on_press(Message::ChDir(Some(b.to_path_buf()))),
                    )
                },
            );

        let scroll = Scrollable::new(&mut self.scroll)
            .push(dirs)
            .push(Space::new(Length::Units(0), Length::Units(15)))
            .push(files)
            .height(Length::FillPortion(12))
            .scroller_width(30)
            .scrollbar_width(40);

        Column::new()
            .push(dir_view)
            .push(button_row)
            .push(scroll)
            .push(confirm_bar)
            .into()
    }
}
