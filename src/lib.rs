// These can be their own .rs file OR
// a named directory with mod.rs + other files
// see: https://doc.rust-lang.org/book/crates-and-modules.html
// see: 'tests' module (some things need pub that tests doesnt mind priv)
// see: 'reporting' module
pub mod control;
pub mod reporting;
pub mod z3_interface;
pub mod weakest_precondition;
pub mod parser;

#[cfg(test)]
mod tests;
