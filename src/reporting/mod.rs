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
use log::{LogRecord, LogLevelFilter};
use env_logger::LogBuilder;

//Sets up the Reporting Modulle
pub fn init() {
	let format = |record: &LogRecord| {
		format!("{} - {}", record.level(), record.args())
	};

	let mut builder = LogBuilder::new();
	builder.format(format).filter(None, LogLevelFilter::Info);

	if env::var("RUST_LoG").is_ok() {
		println!("RUST_LoG must be ok");
		builder.parse(&env::var("RUST_LOG").unwrap());
	}

	builder.init().unwrap(); 

//	error!("error message");
//	warn!("This is a warning message");
//	info!("info message");

	println!("Next test");
}

//code [demo]
pub fn demo() {
    println!("reporting - reporting in");
}

/// Reports successful rustproof behavior to the compiler
/// # Arguments
/// * 'msg' - The message to report
pub fn success() {
    unimplemented!();
}

/// Reports noteworthy rustproof behavior to the compiler
/// # Arguments
/// * 'msg' - The message to report
pub fn warning(val: String) {
	warn!("{}", val);
}

/// Reports errors to the compiler
/// # Arguments
/// * 'msg' - The message to report
pub fn error(val: String) {
    error!("{}", val);
}

/// Reports errors to the compiler
/// # Arguments
/// * 'msg' - The message to report
pub fn debug(val: String) {
    debug!("{}", val);
}

/// Reports errors to the compiler
/// # Arguments
/// * 'msg' - The message to report
pub fn info(val: String) {
    info!("{}", val);
}

/// Reports errors to the compiler
/// # Arguments
/// * 'msg' - The message to report
pub fn trace(val: String) {
    trace!("{}", val);
}
