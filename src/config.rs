use std::vec::Vec;
use std::path::PathBuf;

pub type PathList = Vec<PathBuf>;

pub struct Config {
	pub filenames: PathList,
}

impl Config {
	pub fn new(files: &Vec<String>) -> Config {
		let mut paths: PathList = Vec::new();
		for f in files {
			paths.push(PathBuf::from(f))
		}
		Config {
			filenames: paths,
		}
	}
}
