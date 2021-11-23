//! `monat` is a  Unix shell auxiliary command focusing on the navigation of
//! the file system, especially for working in big projects. Think of a
//! scenario where you are at the root directory of a big project, and there
//! are files lying in many deep directories. It's common to visit the files,
//! rename or move them, run a command in some sub-directories and come back to
//! the root, etc. It would be **tedious to input the long path prefix again
//! and again**.
//!
//! Homepage: https://github.com/Pavinberg/monat


mod monat;

use crate::monat::pathepitome::PathEpitomeParseError;
use std::fmt;
use std::path::PathBuf;
use structopt::StructOpt;
use monat::config::Config;
use monat::histmanager::HistManager;
use monat::pathepitome::PathEpitome;
use monat::run;

/// StructOpt for the command
#[derive(Debug, StructOpt)]
struct Opt {
	#[structopt(short, long, help="Run another command")]
	command: Option<String>,
	
	#[structopt(short, long, help="Toggle to use local monat history")]
	local: bool,
	
	#[structopt(short, long, name="directory", help="dive down into a directory and go back (usually used along with -c)")]
	dive: Option<String>,
	
	#[structopt(name = "path1", help="If only pass path1, act as `ls` command")]
	from: Option<String>,
	
	#[structopt(name = "path2", help="If pass 2 paths, act as `mv` command")]
	to: Option<String>,
}

/// Main function of the program
fn main() {
	let opt = Opt::from_args();
	if let Err(error) = start(opt) {
		handle_error(error);
	}
}

/// An Error type to handle all the error threw in the program
pub enum MonatError {
	IOError(std::io::Error),
	ParseError(PathEpitomeParseError),
	CommandNotFoundError(String),
}

impl fmt::Display for MonatError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self)
	}
}

impl From<std::io::Error> for MonatError {
	fn from(err: std::io::Error) -> MonatError {
		MonatError::IOError(err)
	}
}

impl From<PathEpitomeParseError> for MonatError {
	fn from(err: PathEpitomeParseError) -> MonatError {
		MonatError::ParseError(err)
	}
}

/// start the routine
fn start(opt: Opt) -> Result<(), MonatError> {
	let conf = Config::new(opt.local);
	let mut histmanager = HistManager::new(conf);

	let current_dir = std::env::current_dir().unwrap();
	if let Some(dive_path) = opt.dive {
		let dp = PathEpitome::new(&dive_path, &mut histmanager)?.get_path();
		if let Err(e) = std::env::set_current_dir(dp) {
			return Err(MonatError::IOError(e));
		}
	}
		
	let paths = parse_paths(&mut histmanager, opt.from, opt.to)?;

	if opt.local {
		println!("Toggled to local monat");
	}
	
	if let Some(cmd) = opt.command {
		// let msg = format!("Run command '{}'", &cmd);
		// println!("\x1b[1;30mINFO: {}\x1b[0m", msg);
		run::run_cmd(&cmd, paths)?;
	} else {
		match paths.len() {
			0 => { // print history
				histmanager.pretty_print();
			},
			1 => { // ls
				// let msg = format!("List: '{}'", paths[0].display());
				// println!("\x1b[1;30mINFO: {}\x1b[0m", msg);
				run::run_ls(&paths[0])?;
			},
			2 => { // mv
				// let msg = format!("Move from: '{}' to '{}'",
				// 				  paths[0].display(),
				// 				  paths[1].display()
				// );
				// println!("\x1b[1;30mINFO: {}\x1b[0m", msg);
				run::run_move_two(&paths[0], &paths[1])?;
			}
			_ => {
				eprintln!("Unhandled paths number");
			}
		}
	}
	let _ = std::env::set_current_dir(current_dir);

	histmanager.save();
	
	Ok(())
}

/// Parse the paths. The paths won't all be None because that case will be handled before. 
fn parse_paths(histmanager: &mut HistManager, from: Option<String>, to: Option<String>) -> Result<Vec<PathBuf>, PathEpitomeParseError> {
	let mut ret = Vec::<PathBuf>::new();
	if let Some(from) = from {
		let from = PathEpitome::new(&from, histmanager)?;
		histmanager.set_former_prefix(from.prefix());
		ret.push(from.get_path());
	}
	if let Some(to) = to {
		let to = PathEpitome::new(&to, histmanager)?;
		ret.push(to.get_path());
	}
	Ok(ret)
}

/// Handle all the MonatError
fn handle_error(error: MonatError) {
	match error {
		MonatError::IOError(e) => {
			match e.kind() {
				std::io::ErrorKind::NotFound => {
					eprintln!("{}", e);
				}
				_ => {
					eprintln!("Unhandled IO error: {}", e);
				}
			}
		},
		MonatError::ParseError(e) => {
			eprintln!("{}", e);
		},
		MonatError::CommandNotFoundError(cmd) => {
			eprintln!("CommandNotFoundError:  command '{}' not found", cmd);
		}
	}
}
