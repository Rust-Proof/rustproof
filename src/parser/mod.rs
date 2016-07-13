// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//#[macro_use]
//extern crate rustc;
//extern crate syntax;
//extern crate rustc_plugin;

mod lalrpop; // FIXME: Rename module

use rustc_plugin::Registry;
use syntax::ast::{MetaItem, Item, ItemKind, MetaItemKind, LitKind};
use syntax::ext::base::{ExtCtxt, Annotatable};
use syntax::ext::base::SyntaxExtension::MultiDecorator;
use syntax::codemap::Span;
use syntax::parse::token::intern;
use syntax::ptr::P;
use super::dev_tools; // FIXME: remove for production
use super::Attr;
use rustc::mir::repr::{Mir, BasicBlock, BasicBlockData};
use expression::Predicate;
use expression::Term;
use expression::AndData;
use expression::OrData;
use expression::ImpliesData;
use expression::IntegerComparisonData;
use expression::IntegerComparisonOperator;
use expression::SignedBitVectorData;
use std::str::FromStr;

/// Parses function information from an *Annotatable* associated with an attribute.
/// *builder* is passed by reference
pub fn parse_function(builder: &mut Attr, item: &Annotatable) {
    match item {
        &Annotatable::Item(ref x) => {
            //get function name
            builder.func_name = x.ident.to_string();
            //get span
            builder.func_span = Some(x.span);
            //dev_tools::print_type_of(&x.node);
            match x.node {
                ItemKind::Fn(ref a, ref b, ref c, ref d, ref e, ref block) => {
                    builder.func = Some(block.clone());
                },
                _ => {}
            }
        },
        _ => {}
    }
}

/// Parses attribute information from a *MetaItem* associated with an attribute.
/// *builder* is passed by reference
pub fn parse_attribute(builder: &mut Attr, meta: &MetaItem) {
    match meta.node {
        // FIXME: at the moment, error out if there are no arguments to the attribute
        MetaItemKind::List(ref attribute_name, ref args) => {
            match args.len() {
                1 => {
                    panic!("You must provide pre AND post conditions.");
                    /*
                    println!("Found 1 argument:\n");
                    println!("{:?}\n", args[0]);
                    match args[0].node {
                        MetaItemKind::NameValue(ref x, ref y) =>
                            {
                                if x=="pre" {
                                    builder.pre = Some(y.node.clone());
                                } else if x=="post" {
                                    builder.post = Some(y.node.clone());
                                } else {
                                    panic!("expecting pre and/or post condition {} provided.", x);
                                }
                            },
                        _ => {},
                    }
                    */
                },
                2 => {
                    //println!("Found 2 arguments:\n");
                    //println!("{:?}\n", args[0]);
                    //println!("{:?}\n", args[1]);
                    match args[0].node {
                        MetaItemKind::NameValue(ref x, ref y) => {
                            if x!="pre" { panic!("The first argument must be 'pre'. {} was provided.", x); }
                            //get argument
                            match y.node {
                                super::syntax::ast::LitKind::Str(ref x, ref y) => {
                                    builder.pre_str = x.to_string();
                                }
                                _ => {}
                            }
                            //get span
                            builder.pre_span = Some(y.span);
                        },
                        _ => {},
                    }
                    match args[1].node {
                        MetaItemKind::NameValue(ref x, ref y) => {
                            if x!="post" { panic!("The second argument must be 'post'. {} was provided.", x); }
                            //get argument
                            match y.node {
                                super::syntax::ast::LitKind::Str(ref x, ref y) => {
                                    builder.post_str = x.to_string();
                                }
                                _ => {}
                            }
                            //get span
                            builder.post_span = Some(y.span);
                        },
                        _ => {},
                    }
                },
                _ => {
                    panic!("Too many arguments found for #[condition]; must have pre and/or post conditions");
                }
            }
        },
        _ => {
            panic!("Invalid arguments for #[condition]; did you add a pre and/or post condition?");
        }
    }


    // FIXME: clone?
    //println!("precondition {:?}", builder.clone().pre.unwrap());
    //println!("postcondition {:?}", builder.clone().post.unwrap());
}

// FIXME: Needs implementing
pub fn parse_mir(builder: &Attr, mir: &Mir) {
    //println!("\nfn name: {:#?}", builder.func_name);
}


pub fn demo() {
    println!("parser - reporting in");
}

pub fn parse_condition(condition: &str) -> Predicate {
    match lalrpop::parse_P1(condition) {
        Ok(p) => {
            return p;
        },
        Err(e) => {
            panic!("Error parsing condition: {:?}", e);
        }
    }
}