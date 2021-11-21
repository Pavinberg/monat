//! Configuration of the `monat` program

extern crate shellexpand;

use std::fs;
use std::path::PathBuf;
use std::collections::VecDeque;

const CONFIG_DIR_HOME: &'static str = "~/.monat/";
const CONFIG_DIR_LOCAL: &'static str = "./.monat/";
const HIST_FILE_NAME: &'static str = "history";

pub fn local_history_file() -> PathBuf {
	PathBuf::from(String::from(CONFIG_DIR_LOCAL) + HIST_FILE_NAME)
}

pub fn home_history_file() -> PathBuf {
	PathBuf::from(String::from(shellexpand::tilde(CONFIG_DIR_HOME)) + HIST_FILE_NAME)
}

fn load_hist_from(filename: PathBuf) -> Option<VecDeque<PathBuf>> { 
	if filename.exists() {
		let content = fs::read_to_string(filename).unwrap();
		let records: VecDeque<PathBuf> = content.lines()
			.filter(|line| line.trim() != "")
			.map(|line| PathBuf::from(line)).collect();
		Some(records)
	} else {
		None
	}
}

fn save_hist_to(filename: PathBuf, records: Vec<String>) {
	fs::write(filename, records.join("\n") + "\n").unwrap();
}

enum HistFileLocation {
	Local,
	Home,
	Neither,
}

pub struct Config {
	pub max_records_num: usize,
	hist_location: HistFileLocation, // which history file to use (local or home)
}

impl Default for Config {
	fn default() -> Config {
		Config {
			max_records_num: 9,
			hist_location: HistFileLocation::Home,
		}
	}
}

impl Config {
	pub fn new(use_local: bool) -> Self {
		// create ~/.monat/ if it doesn't exist
		fs::create_dir_all(shellexpand::tilde(CONFIG_DIR_HOME).into_owned())
			.expect(&format!("Failed to create monat directory: {}", CONFIG_DIR_HOME));
		if use_local {
			fs::create_dir_all(CONFIG_DIR_LOCAL)
				.expect(&format!("Failed to create monat directory: {}", CONFIG_DIR_LOCAL));
			fs::write(local_history_file(), "").unwrap();
		}
		if use_local || local_history_file().exists() {
			Config { hist_location: HistFileLocation::Local, ..Default::default() }
		} else if home_history_file().exists() {
			Config { hist_location: HistFileLocation::Home, ..Default::default() }
		} else {
			Config { hist_location: HistFileLocation::Neither, ..Default::default() }
		}
		
	}

	pub fn use_local(&self) -> bool {
		// whther this configuration use local monat directory
		match self.hist_location {
			HistFileLocation::Local => true,
			_ => false,
		}
	}

	/// Load history from file.
	/// Load local history file preferentially, else load file in $HOME/.monat
	pub fn load_history_records(&self) -> VecDeque<PathBuf> {
		// Check if history file exists in current location
		match self.hist_location {
			HistFileLocation::Local => load_hist_from(local_history_file()).unwrap(),
			HistFileLocation::Home => load_hist_from(home_history_file()).unwrap(),
			HistFileLocation::Neither => VecDeque::new()
		}
	}

	/// Save the history records to file
	pub fn save_history_records(&self, records: &VecDeque<PathBuf>) {
		let (filename, record_string): (PathBuf, Vec<String>) = match self.hist_location {
			HistFileLocation::Local => {
				(
					local_history_file(),
					records.iter().map(
						|record| record.to_str().unwrap().to_string()
					).collect()
				)
			}
			_ => {
				(
					home_history_file(),
					records.iter()
						.map(|record| fs::canonicalize(record).unwrap_or(PathBuf::new()) // use absolute path
							 .to_str().unwrap().to_string())
						.filter(|s| s != "") // If a path in old history was removed, filter it out
						.collect()
				)		
			}
		};
		save_hist_to(filename, record_string);
	}
}
