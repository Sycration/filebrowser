use std::path::PathBuf;

///prints out the path to stdout and gracefully shuts down the program
pub fn quit(path: PathBuf) {
	println!("\"{}\"", path.to_str().unwrap());
	std::process::exit(0);
}
