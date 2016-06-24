// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, and Drew Gohman.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// trash code [demo]
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
pub fn warning() {
    unimplemented!();
}

/// Reports errors to the compiler
/// # Arguments
/// * 'msg' - The message to report
pub fn error() {
    unimplemented!();
}
