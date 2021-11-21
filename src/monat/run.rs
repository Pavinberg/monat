//! Finally execute the program. 

use std::path::Path;
use std::path::PathBuf;
use std::fs;
use std::io::{Error, Write};
use std::process::Command;

pub fn run_ls(path: &PathBuf) -> Result<(), Error> {
	let meta = fs::metadata(path)?;
	if meta.is_file() { // this is a file, simply print it
		println!("{}", path.display());
	} else {
		// TODO: print more information, just like `ls -l`
		let sub_files = fs::read_dir(path)?;
		for f in sub_files {
			println!("{}", &f.unwrap().path().display());
		}
	}
	Ok(())
}

pub fn run_move_two(from: &PathBuf, to: &PathBuf) -> Result<(), Error> {
	let fto = &mut (to.clone());
	if to.is_dir() {
		*fto = Path::join(to, from.file_name().unwrap());
	}
	let res = fs::rename(from, fto);
	if let Err(e) = res {
		println!("{:?}", e);
		Err(e)
	} else {
		res
	}
	
}


pub fn run_cmd(cmd: &str, args: Vec<PathBuf>) {
	let mut cmd_to_run = Command::new(&cmd);
	for arg in args {
		cmd_to_run.arg(arg);
	}
	let output = cmd_to_run.output().expect(&format!("Command {} failed", &cmd));
	if !output.status.success() {
		std::io::stderr().write_all(&output.stderr).unwrap();
	}
	std::io::stdout().write_all(&output.stdout).unwrap();
}
