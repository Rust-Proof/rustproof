// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Unused, uncomment if they are needed
// use errors::{ColorConfig, Handler};
// use syntax::codemap::CodeMap;
// use std::rc::Rc;

// Warning macro
macro_rules! rp_warn {
    ($fmt:expr) => ({
        let codemap = Rc::new(CodeMap::new());
        let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(codemap.clone()));
        let str = format!(concat!($fmt, "\n"));
        handler.warn(&str);
    });
    ($fmt:expr, $($arg:tt)*) => ({
        let codemap = Rc::new(CodeMap::new());
        let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(codemap.clone()));
        let str = format!(concat!($fmt, "\n"), $($arg)*);
        handler.warn(&str);
    });
}

// Error macro
macro_rules! rp_error {
    ($fmt:expr) => ({
        let codemap = Rc::new(CodeMap::new());
        let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(codemap.clone()));
        let str = format!(concat!($fmt, "\n"));
        handler.err(&str);
	process::exit(1);
    });
    ($fmt:expr, $($arg:tt)*) => ({
        let codemap = Rc::new(CodeMap::new());
        let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(codemap.clone()));
        let str = format!(concat!($fmt, "\n"), $($arg)*);
        handler.err(&str);
	process::exit(1);
    });
}
