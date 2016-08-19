// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Parses rustproof attribute definitions.

extern crate syntax;

mod expression_parser;

use syntax::ast::{MetaItemKind, Attribute_};
use syntax::codemap::{Spanned, CodeMap};
use expression::{Expression, ty_check};
use std::process;
use std::rc::Rc;
use errors::{ColorConfig, Handler};

/// Analyzes an attribute on a function in the compiled code, and if the attribute is "condition",
/// ensures correct usage. If usage is correct, it stores the argument strings.
///
/// # Arguments:
/// * `pre_string` - Empty string. Will contain a user-submitted precondition if found.
/// * `post_string` - Empty string. Will contain a user-submitted postcondition if found.
/// * `attr` - The attribute being analyzed.
///
/// # Remarks:
/// * Currently supported `ConstInt`: `i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32`, `u64`, `bool`
///
pub fn parse_attribute(pre_string: &mut String,
                       post_string: &mut String,
                       attr: &Spanned<Attribute_>) {
    if let MetaItemKind::List(ref attribute_name, ref args) = attr.node.value.node {
        // Ignore if not a condition attribute
        if attribute_name == "condition" {
            // Only accept if exactly 2 arguments
            if args.len() != 2 {
                rp_error!("Condition attribute must have exactly 2 arguments.");
            }
            // Parse the first argument
            if let MetaItemKind::NameValue(ref i_string, ref literal) = args[0].node {
                if i_string != "pre" {
                    rp_error!( "The first argument must be named \"pre\". {} was provided.",
                               i_string);
                }
                // Get the argument
                if let syntax::ast::LitKind::Str(ref i_string, _) = literal.node {
                    *pre_string = i_string.to_string();
                } else {
                    rp_error!("Conditions must be strings. \
                              Try wrapping conditions in quotation marks.");
                }
            } else {
                        rp_error!("The first argument must be named \"pre\".");
            }
            // Parse the second argument
            if let MetaItemKind::NameValue(ref i_string, ref literal) = args[1].node {
                if i_string != "post" {
                    rp_error!( "The second argument must be named \"post\". {} was provided.",
                               i_string);
                }
                // Get the argument
                if let syntax::ast::LitKind::Str(ref i_string, _) = literal.node {
                    *post_string = i_string.to_string();
                } else {
                    rp_error!("Conditions must be strings. \
                              Try wrapping conditions in quotation marks.");
                }
            } else {
                rp_error!("The second argument must be named \"post\".");
            }
        } // Ignore if not a condition attribute
    }
}

/// Calls the expression parser on a given precondition or postcondition.
///
/// # Arguments:
/// * `condition` - A user-submitted string
///
/// # Return:
/// * If `condition` is valid, an Expression representing it.
///
/// # Remarks:
/// * Current supported types: `i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32`, `u64`, `bool`
pub fn parse_condition(condition: &str) -> Expression {
    match expression_parser::parse_E1(condition) {
        Ok(e) => {
            match ty_check(&e) {
                Ok(_) => return e,
                Err(s) => rp_error!("{}", s),
            }
        },
        Err(e) => rp_error!("Error parsing condition \"{}\": {:?}", condition, e)
    }
}
