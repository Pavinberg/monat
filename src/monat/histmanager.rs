//! A mangaer that manages history path prefix.
//! 1. Load from history file (local or user's).
//! 2. Return record by index.
//! 3. Append record.
//! 4. Save the new history to file (the same where it was loaded from or create one)

use std::fs;
use std::path::PathBuf;
use std::collections::VecDeque;

use crate::monat::config;

pub struct HistManager {
	conf: config::Config,
	records: VecDeque<PathBuf>,
	changed: bool, // whether new record is added. Used to skip file saving when not chanegd.
	former_prefix: PathBuf, // temporarily store the <path1> prefix in case the <path2> want to use.
}

impl HistManager {
	/// Load history from file.
	/// Load local history file preferentially, else load file in $HOME/.monat
	pub fn new(conf: config::Config) -> Self {
		let records = conf.load_history_records();
		HistManager {
			conf,
			records,
			changed: false,
			former_prefix: PathBuf::new()
		}
	}

	/// Get the history record by index. index starts from 1,
	/// index == 0 return the latest record. Since we use FIFO for now,
	/// latest record is the last record.
	pub fn get(&self, index: usize) -> Option<PathBuf> {
		if index == 0 {
			Some(self.former_prefix.clone())
		} else if index <= self.records.len() {
			Some(self.records[index - 1].clone())
		} else {
			None
		}
	}

	pub fn set_former_prefix(&mut self, former_prefix: PathBuf) {
		self.former_prefix = former_prefix;
	}

	/// Add a record to the history manager if it does not exist.
	/// Use FIFO stategy for now, so simply append to last.
	pub fn add_if_nonexists(&mut self, record: PathBuf) {
		if self.conf.use_local() {
			if record.to_str().unwrap().trim() != "" &&
				!self.records.iter().any(|r| *r == record) { // hasn't added
					self.records.push_back(record);
					self.changed = true;
				}
		} else {
			let mut rec = record.clone();
			if rec == PathBuf::new() {
				rec = PathBuf::from(".");
			}
			let abs_record = fs::canonicalize(rec).unwrap();
			if !self.records.iter().any(|r| *r == abs_record) { // hasn't added
				self.records.push_back(abs_record);
				self.changed = true;
			}
		}
	}

	/// Save the history to file
	pub fn save(&mut self) {
		if self.changed {
			while self.records.len() > self.conf.max_records_num {
				self.records.pop_front();
			}
			self.conf.save_history_records(&self.records);
		}
	}

	pub fn pretty_print(&self) {
		if self.records.len() == 0 {
			println!("[No history]");
		}
		for (i, record) in self.records.iter().enumerate() {
			// TODO: pretty print
			println!("{} -- {}", (i+1), record.to_str().unwrap());
		}
	}
}
