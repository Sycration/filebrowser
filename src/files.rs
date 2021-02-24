use iced::button;
use std::fs::{metadata, read_dir};
use std::path::{Path, PathBuf};

///Read the names of all files in a directory and return them as a vec of paths
pub fn get_list(path: &Path) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut res = Vec::new();
    let mut dirs = Vec::new();
    let path = path.canonicalize().unwrap();
    if let Ok(read_path) = read_dir(path) {
        for path in read_path {
            if let Ok(path) = path {
                if let Ok(v) = metadata(path.path()) {
                    if v.file_type().is_file() {
                        res.push(path.path());
                    } else {
                        dirs.push(path.path());
                    }
                }
            }
        }
    }
    (res, dirs)
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
