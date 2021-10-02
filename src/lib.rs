pub mod config;

use std::fmt::Debug;
use std::path::PathBuf;

pub type FSItemList = Vec<FSItem>;

#[derive(Copy, Clone)]
pub enum OutputType {
	Summary,
	Full
}

#[derive(Debug)]
pub struct FileInfo {
	pub path: PathBuf,
	pub size: u64,
}
impl FileInfo {
	pub fn new(path: &PathBuf) -> FileInfo {
		FileInfo {
			path: path.clone(),
			size: 0,
		}
	}
}

#[derive(Debug)]
pub struct DirInfo {
	pub path: PathBuf,
	pub size: u64,
	pub childs: FSItemList,
}

impl DirInfo {
	pub fn new(path: &PathBuf) -> DirInfo {
		DirInfo {
			path: path.clone(),
			size: 0,
			childs: Vec::new(),
		}
	}
}

#[derive(Debug)]
pub enum FSItem {
	File(FileInfo),
	Dir(DirInfo),
}
