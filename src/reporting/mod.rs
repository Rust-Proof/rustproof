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

// The Warning Macro
macro_rules! rp_warn {
    ($fmt:expr) => ({
        warn!(concat!($fmt, "\n"));
    });
    ($fmt:expr, $($arg:tt)*) => ({
        warn!(concat!($fmt, "\n"), $($arg)*);
    });
}

// The Error Macro
macro_rules! rp_error {
    ($fmt:expr) => ({
        error!(concat!($fmt, "\n"));
	process::exit(1);
    });
    ($fmt:expr, $($arg:tt)*) => ({
        error!(concat!($fmt, "\n"), $($arg)*);
	process::exit(1);
    });
}

// The Debug Macro
macro_rules! rp_debug {
    ($fmt:expr) => ({
        debug!(concat!($fmt, "\n"));
    });
    ($fmt:expr, $($arg:tt)*) => ({
        debug!(concat!($fmt, "\n"), $($arg)*);
    });
}

// The Info Macro
macro_rules! rp_info {
    ($fmt:expr) => ({
        info!(concat!($fmt, "\n"));
    });
    ($fmt:expr, $($arg:tt)*) => ({
        info!(concat!($fmt, "\n"), $($arg)*);
    });
}

// The Trace Macro
macro_rules! rp_trace {
    ($fmt:expr) => ({
        Ok(_) => trace!(concat!($fmt, "\n"));
    });
    ($fmt:expr, $($arg:tt)*) => ({
        trace!(concat!($fmt, "\n"), $($arg)*);
    });
}

//Sets up the Reporting Modulle
pub fn init() {
	let format = |record: &LogRecord| {
		format!("________________________\n{}\n{}\n________________________", record.level(), record.args())
	};

	let mut builder = LogBuilder::new();
	builder.format(format).filter(None, LogLevelFilter::Info);

	if env::var("RUST_LoG").is_ok() {
		println!("RUST_LoG must be ok");
		builder.parse(&env::var("RUST_LOG").unwrap());
	}

	builder.init().unwrap();
}
