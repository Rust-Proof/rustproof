// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
/// * Current supported variable and literal types: i8, i16, i32, i64, u8, u16, u32, u64, bool
///
pub fn parse_attribute(pre_string: &mut String,
                       post_string: &mut String,
                       attr: &Spanned<Attribute_>) {
    match attr.node.value.node {
        MetaItemKind::List(ref attribute_name, ref args) => {
            // Ignore if not a condition attribute
            if attribute_name == "condition" {
                // Only accept if exactly 2 arguments
                if args.len() != 2 {
                    rp_error!("Condition attribute must have exactly 2 arguments.");
                }
                // Parse the first argument
                match args[0].node {
                    MetaItemKind::NameValue(ref i_string, ref literal) => {
                        if i_string != "pre" {
                            rp_error!(
                                "The first argument must be \"pre\". {} was provided.",
                                i_string
                            );
                        }
                        // Get the argument
                        match literal.node {
                            syntax::ast::LitKind::Str(ref i_string, _) => {
                                *pre_string = i_string.to_string();
                            }
                            _ => {}
                        }
                    },
                    _ => {},
                }
                // Parse the second argument
                match args[1].node {
                    MetaItemKind::NameValue(ref i_string, ref literal) => {
                        if i_string != "post" {
                            rp_error!(
                                "The second argument must be \"post\". {} was provided.",
                                i_string
                            );
                        }
                        // Get the argument
                        match literal.node {
                            syntax::ast::LitKind::Str(ref i_string, _) => {
                                *post_string = i_string.to_string();
                            }
                            _ => {}
                        }
                    },
                    _ => {},
                }
            }
        },
        _ => {}
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
/// * Current supported variable and literal types: i8, i16, i32, i64, u8, u16, u32, u64, bool
///
pub fn parse_condition(condition: &str) -> Expression {
    match expression_parser::parse_E1(condition) {
        Ok(e) => {
            match ty_check(&e) {
                Ok(_) => {
                    return e;
                },
                Err(s) => {
                    rp_error!("{}", s);
                }
            }
        },
        Err(e) => {
            rp_error!("Error parsing condition \"{}\": {:?}", condition, e);
        }
    }
}
