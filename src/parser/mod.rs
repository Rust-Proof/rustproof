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

pub fn parse_predicate_from_string(condition: String) -> Predicate {
    let mut v = Vec::new();
    for s in condition.split_whitespace() {
        println!("{}", s);
        v.push(s);
    }

    parse_predicate_from_vec(v)
}

pub fn parse_predicate_from_vec(v: Vec<&str>) -> Predicate {
    let v_length = v.len();
    if v_length >= 1 {
        match v[0] {
            //In parentheses
            "(" => {
                unimplemented!();
            } 
            //Unary boolean operator
            "!" => {
                unimplemented!();
            }
            //Unary integer operator
            //Integer literal
            _ if is_integer(v[0]) => {
                //Store the literal
                let first_term = Term::SignedBitVector( SignedBitVectorData {
                    size: 64, value: v[0].parse().unwrap()
                } );

                if 3 >= v_length {
                    //ERROR
                    unimplemented!();
                } else {
                    //Look for integer operator or next boolean operator
                    match v[1] {
                        //Integer comparison operator
                        ">" | ">=" | "<" | "<=" | "==" | "!=" => {
                            let mut remaining = Vec::new();

                            for j in (2)..v_length {
                                remaining.push(v[j]);
                            }
                            return Predicate::IntegerComparison( IntegerComparisonData {
                                op: return_boolean_operator(v[1]),
                                t1: Box::new(first_term),
                                t2: Box::new(parse_term_from_vec(remaining))
                            } );
                        },
                        _ => {
                            //ERROR
                            unimplemented!();
                        }
                    }
                }
            }
            //Boolean literal
            "true" | "false" => {
                //Store the literal
                let first_predicate = Predicate::BooleanLiteral( v[0].parse().unwrap() );

                //Could be single boolean literal
                if 1 >= v_length {
                    return first_predicate;
                } else {
                    //Look for boolean operator
                    match v[1] {
                        //Binary boolean operator
                        "&&" => {
                            let mut remaining = Vec::new();
                            if 2 >= v_length {
                                //ERROR
                                unimplemented!();
                            }
                            for j in (2)..v_length {
                                remaining.push(v[j]);
                            }
                            return Predicate::And( AndData {
                                p1: Box::new(first_predicate),
                                p2: Box::new(parse_predicate_from_vec(remaining))
                            } );
                        },
                        "||" => {
                            let mut remaining = Vec::new();
                            if 2 >= v_length {
                                //ERROR
                                unimplemented!();
                            }
                            for j in (2)..v_length {
                                remaining.push(v[j]);
                            }
                            return Predicate::Or( OrData {
                                p1: Box::new(first_predicate),
                                p2: Box::new(parse_predicate_from_vec(remaining))
                            } );
                        },
                        "->" => {
                            let mut remaining = Vec::new();
                            if 2 >= v_length {
                                //ERROR
                                unimplemented!();
                            }
                            for j in (2)..v_length {
                                remaining.push(v[j]);
                            }
                            return Predicate::Implies( ImpliesData {
                                p1: Box::new(first_predicate),
                                p2: Box::new(parse_predicate_from_vec(remaining))
                            } );
                        }
                        _ => {
                            //ERROR
                            unimplemented!();
                        }
                    }
                }
            }
            //Variable name
            _ => {
                //ERROR
                unimplemented!();
            }
        }
    }

    unimplemented!();
}

pub fn parse_term_from_vec(v: Vec<&str>) -> Term {
    unimplemented!();
}

pub fn is_integer(s: &str) -> bool {
    match i64::from_str(s) {
        Ok(..) => {
            return true;
        },
        Err(..) => {
            return false;
        }
    }
}

pub fn return_boolean_operator(s: &str) -> IntegerComparisonOperator {
    match s {
        ">" => {
            return IntegerComparisonOperator::GreaterThan;
        },
        "<" => {
            return IntegerComparisonOperator::LessThan;
        },
        ">=" => {
            return IntegerComparisonOperator::GreaterThanOrEqual;
        },
        "<=" => {
            return IntegerComparisonOperator::LessThanOrEqual;
        },
        "==" => {
            return IntegerComparisonOperator::Equal;
        },
        "!=" => {
            return IntegerComparisonOperator::NotEqual;
        },
        _ => {
            //ERROR
            unimplemented!();
        }
    }
}