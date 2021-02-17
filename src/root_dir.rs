use std::path::PathBuf;


///The root on anything but windows (thanks microsoft) is `/`
#[cfg(not(Windows))]
pub fn root_dir() -> PathBuf {
    PathBuf::from(r"/")
}

///Microsoft will not behave
#[cfg(Windows)]
pub fn root_dir() -> PathBuf {
    PathBuf::from(r"c:\")
}
