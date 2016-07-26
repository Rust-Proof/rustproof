// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::env;
use std::process;
use log::{LogRecord, LogLevelFilter};
use env_logger::LogBuilder;
use term;


//Sets up the Reporting Modulle
pub fn init() {
	let format = |record: &LogRecord| {
		format!("{}", record.args())
	};

	let mut builder = LogBuilder::new();
	builder.format(format).filter(None, LogLevelFilter::Info);

	if env::var("RUST_LoG").is_ok() {
		println!("RUST_LoG must be ok");
		builder.parse(&env::var("RUST_LOG").unwrap());
	}

	builder.init().unwrap();

	//error("error message");
	//info("info message");

	//println!("Next test");
}



/// Reports noteworthy rustproof behavior to the compiler
/// # Arguments
/// * 'msg' - The message to report
pub fn warn(val: &'static str) {
	let mut terminal = term::stdout().unwrap();
	terminal.fg(term::color::YELLOW).unwrap();
	terminal.attr(term::Attr::Bold).unwrap();
	
	write!(terminal, "WARNING: ").unwrap();
	terminal.reset().unwrap();
	terminal.flush();
	warn!("{}", val);
}

/// Reports errors to the compiler
/// # Arguments
/// * 'msg' - The message to report
pub fn error(val: &'static str) {
	let mut terminal = term::stdout().unwrap();
	terminal.fg(term::color::RED).unwrap();
	terminal.attr(term::Attr::Bold).unwrap();
	
	write!(terminal, "ERROR: ").unwrap();
	terminal.reset().unwrap();
	terminal.flush();
	error!("{}", val);
	process::exit(1);
}

/// Reports errors to the compiler
/// # Arguments
/// * 'msg' - The message to report
pub fn debug(val: &'static str) {
	let mut terminal = term::stdout().unwrap();
	terminal.fg(term::color::RED).unwrap();
	terminal.attr(term::Attr::Bold).unwrap();
	
	write!(terminal, "DEBUG: ").unwrap();
	terminal.reset().unwrap();
	terminal.flush();
	debug!("{}", val);
}



/// Reports errors to the compiler
/// # Arguments
/// * 'msg' - The message to report
pub fn info(val: &'static str) {
	let mut terminal = term::stdout().unwrap();
	terminal.fg(term::color::WHITE).unwrap();
	terminal.attr(term::Attr::Bold).unwrap();
	
	write!(terminal, "INFO: ").unwrap();
	terminal.reset().unwrap();
	terminal.flush();
	info!("{}", val);
}

/// Reports errors to the compiler
/// # Arguments
/// * 'msg' - The message to report
pub fn trace(val: &'static str) {
	let mut terminal = term::stdout().unwrap();
	terminal.fg(term::color::GREEN).unwrap();
	terminal.attr(term::Attr::Bold).unwrap();
	
	write!(terminal, "TRACE: ").unwrap();
	terminal.reset().unwrap();
	terminal.flush();
	trace!("{}", val);
}
