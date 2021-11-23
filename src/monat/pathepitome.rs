//! A PathEpitome is a path with comma expression.
//! 
//! ```
//! ,1/foo
//!```
//!
//! This struct contains functions to parse the PathEpitome and return the
//! PathBuf after expansion.

extern crate path_absolutize;

use std::error::Error;
use std::fmt;
use std::path::Path;
use std::path::PathBuf;
use path_absolutize::*;
use regex::Regex;

use crate::monat::histmanager::HistManager;

pub struct PathEpitome {
	has_epitome: bool,
	hist_idx: usize,
	prefix: PathBuf,
	suffix: PathBuf,
}

impl PathEpitome {
	fn parse_path_str(path: &str) -> Result<(usize, &str), PathEpitomeParseError> {
		let hist_idx_pattern = Regex::new(r"^,[0-9]+").unwrap();
		let mut hist_idx: usize = 0;
		let mut suffix_begin = 1;
		if let Some(hist_idx_match) = hist_idx_pattern.find(path) {
			let hist_idx_str = &path[
				hist_idx_match.start() + 1 .. hist_idx_match.end()];
			hist_idx = hist_idx_str
				.to_string().parse::<usize>().unwrap(); // should never panic because checked by regex
			suffix_begin = hist_idx_match.end();
		} // else use default history index 0

		if suffix_begin < path.len() {
			if let Some(suffix_str) = path[suffix_begin..].strip_prefix("/") {
				Ok((hist_idx, suffix_str))
			} else {
				Err(PathEpitomeParseError::SyntaxError(String::from(
					"A slash (/) is expected after comma expression")))
			}
		} else { // only got r",[0-9]?"
			Ok((hist_idx, ""))
		}
	}

	fn simplify_relative_path(path: &PathBuf) -> PathBuf {
		let current_dir = Path::new(".").absolutize().unwrap();
		path.absolutize().unwrap().strip_prefix(current_dir).unwrap().to_path_buf()
	}
	
	pub fn new(path: &str, histmanager: &mut HistManager) -> Result<Self, PathEpitomeParseError> {
		// if string starts with comma, this is a PathEpitome
		if path.starts_with(",") {
			let (hist_idx, suffix_str) = PathEpitome::parse_path_str(path)?;
			let prefix = histmanager.get(hist_idx)
				.ok_or(PathEpitomeParseError::IndexOutOfRangeError(hist_idx))?;
			histmanager.add_if_nonexists(PathEpitome::simplify_relative_path(&prefix));
			Ok(PathEpitome {
				has_epitome: true,
				hist_idx,
				prefix,
				suffix: PathBuf::from(suffix_str.to_string())
			})
				
		} else {
			// regular path
			let pathbuf = PathBuf::from(path);
			let prefix = pathbuf.parent().unwrap_or(Path::new(".")).to_path_buf();
			histmanager.add_if_nonexists(PathEpitome::simplify_relative_path(&prefix));
			if let Some(suffix) = PathBuf::from(path).file_name() {
				Ok(PathEpitome {
					has_epitome: false,
					hist_idx: 0,
					prefix,
					suffix: PathBuf::from(suffix)
				})
			} else { // usually when path is "."
				Ok(PathEpitome {
					has_epitome: false,
					hist_idx: 0,
					prefix,
					suffix: pathbuf
				})
			}
		}
	}
	
	pub fn get_path(&self) -> PathBuf  {
		let mut path = self.prefix.clone();
		path.push(&self.suffix);
		path.absolutize().unwrap().to_path_buf()
	}

	pub fn prefix(&self) -> PathBuf {
		self.prefix.clone()
	}
}

impl fmt::Debug for PathEpitome {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.has_epitome {
			write!(f, "hist_idx={}, suffix=\"{}\"", self.hist_idx, self.suffix.to_str().unwrap())
		} else {
			write!(f, "\"{}\"", self.suffix.to_str().unwrap())
		}
	}
}

#[derive(Debug)]
pub enum PathEpitomeParseError {
	SyntaxError(String),
	IndexOutOfRangeError(usize),
}

impl Error for PathEpitomeParseError {}

impl fmt::Display for PathEpitomeParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			PathEpitomeParseError::SyntaxError(e) => write!(f, "PathEpitomeParseError: {}", e),
			PathEpitomeParseError::IndexOutOfRangeError(i) =>
				write!(f, "IndexOutOfRangeError: index of {} is out of history records range", i),
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_path_str_wo_idx() {
		let path = ",/abc/def";
		if let Ok((idx, suffix_str)) = PathEpitome::parse_path_str(&path) {
			assert_eq!(idx, 0);
			assert_eq!(suffix_str, "abc/def");
		} else {
			assert!(false, "Error parsing {}", path);
		}
	}

	#[test]
	fn test_parse_path_str_w_idx() {
		let path = ",1/abc/def";
		if let Ok((idx, suffix_str)) = PathEpitome::parse_path_str(&path) {
			assert_eq!(idx, 1);
			assert_eq!(suffix_str, "abc/def");
		} else {
			assert!(false, "Error parsing {}", path);
		}
	}

	#[test]
	fn test_parse_path_str_error() {
		let path = ",abc/def";
		if let Err(PathEpitomeParseError::SyntaxError(msg)) = PathEpitome::parse_path_str(&path) {
			assert_eq!(msg, "A slash (/) is expected after comma expression");
		} else {
			assert!(false, "Should raise 'PathEpitomeParseError::SyntaxError' when parsing {}", path);
		}
	}

}
