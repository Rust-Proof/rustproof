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
extern crate term;

mod predicate_parser;

use syntax::ast::{MetaItemKind, Attribute_};
use syntax::codemap::Spanned;

use super::Attr;
use expression::Expression;
use std::process;

// Checks for the applicable "condition" attribute and ensures correct usage. If usage is correct, it stores the argument strings.
pub fn parse_attribute(builder: &mut Attr, attr: &Spanned<Attribute_>) {
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
                        if i_string != "pre" { rp_error!("The first argument must be \"pre\". {} was provided.", i_string); }
                        // Get the argument
                        match literal.node {
                            syntax::ast::LitKind::Str(ref i_string, _) => {
                                builder.pre_string = i_string.to_string();
                            }
                            _ => {}
                        }
                        // Get the span
                        builder.pre_span = Some(literal.span);
                    },
                    _ => {},
                }
                // Parse the second argument
                match args[1].node {
                    MetaItemKind::NameValue(ref i_string, ref literal) => {
                        if i_string != "post" { rp_error!("The second argument must be \"post\". {} was provided.", i_string); }
                        // Get the argument
                        match literal.node {
                            syntax::ast::LitKind::Str(ref i_string, _) => {
                                builder.post_string = i_string.to_string();
                            }
                            _ => {}
                        }
                        // Get the span
                        builder.post_span = Some(literal.span);
                    },
                    _ => {},
                }
            }
        },
        _ => {}
    }
}

// Calls the predicate parser on a given pre/post condition, and returns a Expression if it is valid.
pub fn parse_condition(condition: &str) -> Expression {
    match predicate_parser::parse_E1(condition) {
        Ok(e) => {
            return e;
        },
        Err(e) => {
            rp_error!("Error parsing condition \"{}\": {:?}", condition, e);
        }
    }
}
