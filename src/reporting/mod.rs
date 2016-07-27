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
        //Set up the Terminal for formatting
        let mut terminal = term::stdout().unwrap();
        terminal.fg(term::color::YELLOW).unwrap();
        terminal.attr(term::Attr::Bold).unwrap();
       
        //Terminal will only work with Write Macro from what I know 
        write!(terminal, "WARNING: ").unwrap();
        terminal.reset().unwrap();
        terminal.flush();
        //Call the Warning log function with sent in message
        warn!(concat!($fmt, "\n"));
    });
    ($fmt:expr, $($arg:tt)*) => ({
        //Set up the Terminal for formatting
        let mut terminal = term::stdout().unwrap();
        terminal.fg(term::color::YELLOW).unwrap();
        terminal.attr(term::Attr::Bold).unwrap();
        
        //Terminal will only work with Write Macro from what I know 
        write!(terminal, "WARNING: ").unwrap();
        terminal.reset().unwrap();
        terminal.flush();
        //Call the Warning log function with sent in message
        warn!(concat!($fmt, "\n"), $($arg)*);
    });
}

// The Error Macro
macro_rules! rp_error {
    ($fmt:expr) => ({
        // Set up the Terminal for formatting
        let mut terminal = term::stdout().unwrap();
        terminal.fg(term::color::RED).unwrap();
        terminal.attr(term::Attr::Bold).unwrap();
       
        // Terminal will only work with Write Macro from what I know 
        write!(terminal, "ERROR: ").unwrap();
        terminal.reset().unwrap();
        terminal.flush();
        // Call the Error log function with sent in message
        error!(concat!($fmt, "\n"));
	    process::exit(1);
    });
    ($fmt:expr, $($arg:tt)*) => ({
        // Set up the Terminal for formatting
        let mut terminal = term::stdout().unwrap();
        terminal.fg(term::color::RED).unwrap();
        terminal.attr(term::Attr::Bold).unwrap();
        
        // Terminal will only work with Write Macro from what I know 
        write!(terminal, "ERROR: ").unwrap();
        terminal.reset().unwrap();
        terminal.flush();
        // Call the Error log function with sent in message
        error!(concat!($fmt, "\n"), $($arg)*);
        // Exit out of program
	    process::exit(1);
    });
}

// The Debug Macro
macro_rules! rp_debug {
    ($fmt:expr) => ({
        //Set up the Terminal for formatting
        let mut terminal = term::stdout().unwrap();
        terminal.attr(term::Attr::Bold).unwrap();
       
        //Terminal will only work with Write Macro from what I know 
        write!(terminal, "DEBUG: ").unwrap();
        terminal.reset().unwrap();
        terminal.flush();
        //Call the Debug log function with sent in message
        debug!(concat!($fmt, "\n"));
    });
    ($fmt:expr, $($arg:tt)*) => ({
        //Set up the Terminal for formatting
        let mut terminal = term::stdout().unwrap();
        terminal.attr(term::Attr::Bold).unwrap();
        
        //Terminal will only work with Write Macro from what I know 
        write!(terminal, "DEBUG: ").unwrap();
        terminal.reset().unwrap();
        terminal.flush();
        //Call the Debug log function with sent in message
        debug!(concat!($fmt, "\n"), $($arg)*);
    });
}

// The Info Macro
macro_rules! rp_info {
    ($fmt:expr) => ({
        //Set up the Terminal for formatting
        let mut terminal = term::stdout().unwrap();
        terminal.attr(term::Attr::Bold).unwrap();
       
        //Terminal will only work with Write Macro from what I know 
        write!(terminal, "INFO: ").unwrap();
        terminal.reset().unwrap();
        terminal.flush();
        //Call the info log function with sent in message
        info!(concat!($fmt, "\n"));
    });
    ($fmt:expr, $($arg:tt)*) => ({
        //Set up the Terminal for formatting
        let mut terminal = term::stdout().unwrap();
        terminal.attr(term::Attr::Bold).unwrap();
        
        //Terminal will only work with Write Macro from what I know 
        write!(terminal, "INFO: ").unwrap();
        terminal.reset().unwrap();
        terminal.flush();
        //Call the info log function with sent in message
        info!(concat!($fmt, "\n"), $($arg)*);
    });
}

// The Trace Macro
macro_rules! rp_trace {
    ($fmt:expr) => ({
        //Set up the Terminal for formatting
        let mut terminal = term::stdout().unwrap();
        terminal.attr(term::Attr::Bold).unwrap();
       
        //Terminal will only work with Write Macro from what I know 
        write!(terminal, "TRACE: ").unwrap();
        terminal.reset().unwrap();
        terminal.flush();
        //Call the Trace log function with sent in message
        trace!(concat!($fmt, "\n"));
    });
    ($fmt:expr, $($arg:tt)*) => ({
        //Set up the Terminal for formatting
        let mut terminal = term::stdout().unwrap();
        terminal.attr(term::Attr::Bold).unwrap();
        
        //Terminal will only work with Write Macro from what I know 
        write!(terminal, "TRACE: ").unwrap();
        terminal.reset().unwrap();
        terminal.flush();
        //Call the Trace log function with sent in message
        trace!(concat!($fmt, "\n"), $($arg)*);
    });
}

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
}
