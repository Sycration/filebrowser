use iced::button;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

///Read the names of all files in a directory and return them as a vec of paths
pub fn get_list_of_files(path: &Path) -> Vec<PathBuf> {
    let mut res = Vec::new();
    let mut dirs = Vec::new();
    if let Ok(read_path) = read_dir(path) {
        for path in read_path {
            if let Ok(path) = path {
                if path.file_type().unwrap().is_dir() {
                    dirs.push(path.path());
                } else {
                    res.push(path.path());
                }
            }
        }
    }
    res.extend(dirs);
    res
}

///Reconciles the amount of button states with the amount of files that should be shown. Avoids more allocation than needed.
pub fn button_list_gen(states: &mut Vec<button::State>, list: &Vec<PathBuf>) {
    if states.len() < list.len() {
        while states.len() != list.len() {
            states.push(button::State::new());
        }
    } else if states.len() > list.len() {
        while states.len() != list.len() {
            states.pop();
        }
    }
}
