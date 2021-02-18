use std::path::PathBuf;
#[cfg(not(Windows))]
pub fn correct_string(path: &PathBuf) -> String {
    if path.is_dir() {
        format!("{}/", path.file_name().unwrap().to_str().unwrap())
    } else {
        path.file_name().unwrap().to_str().unwrap().to_string()
    }
}
#[cfg(Windows)]
pub fn correct_string(path: &PathBuf) -> String {
    if path.is_dir() {
        format!("{}\\", path.file_name().unwrap().to_str().unwrap())
    } else {
        path.file_name().unwrap().to_str().unwrap().to_string()
    }
}
