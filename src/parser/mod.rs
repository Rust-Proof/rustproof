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
extern crate syntax;
//extern crate rustc_plugin;

mod lalrpop; // FIXME: Rename module

use rustc_plugin::Registry;
use syntax::ast::{MetaItem, Item, ItemKind, MetaItemKind, LitKind, Attribute_};
use syntax::ext::base::{ExtCtxt, Annotatable};
use syntax::ext::base::SyntaxExtension::MultiDecorator;
use syntax::codemap::{Span, Spanned};
use syntax::parse::token::intern;
use syntax::ptr::P;
use super::dev_tools; // FIXME: remove for production
use super::Attr;
use expression::Predicate;
use expression::Term;
use expression::AndData;
use expression::OrData;
use expression::ImpliesData;
use expression::IntegerComparisonData;
use expression::IntegerComparisonOperator;
use expression::SignedBitVectorData;
use std::str::FromStr;
use rustc::mir::repr::{Mir, BasicBlock, BasicBlockData, TerminatorKind};
use rustc_data_structures::indexed_vec::Idx;


// FIXME: This needs to be updated; we are no longer using &Annotatable
/// Parses function information from an *Annotatable* associated with an attribute.
/// *builder* is passed by reference
/*
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
*/

pub fn parse_attribute(builder: &mut Attr, attr: &Spanned<Attribute_>) {
    match attr.node.value.node {
        MetaItemKind::List(ref attribute_name, ref args) => {
            //check the attribute is 'condition'
            if attribute_name == "condition" {
                //error if incorrect arg count
                if args.len() != 2 {
                    panic!("condition attribute must have exactly 2 arguments");
                }
                // parse arg 1
                match args[0].node {
                    MetaItemKind::NameValue(ref i_string, ref literal) => {
                        if i_string != "pre" { panic!("The first argument must be 'pre'. {} was provided.", i_string); }
                        //get argument
                        match literal.node {
                            syntax::ast::LitKind::Str(ref i_string, _) => {
                                builder.pre_str = i_string.to_string();
                            }
                            _ => {}
                        }
                        //get span
                        builder.pre_span = Some(literal.span);
                    },
                    _ => {},
                }
                // parse arg 2
                match args[1].node {
                    MetaItemKind::NameValue(ref i_string, ref literal) => {
                        if i_string != "post" { panic!("The second argument must be 'post'. {} was provided.", i_string); }
                        //get argument
                        match literal.node {
                            syntax::ast::LitKind::Str(ref i_string, _) => {
                                builder.post_str = i_string.to_string();
                            }
                            _ => {}
                        }
                        //get span
                        builder.post_span = Some(literal.span);
                    },
                    _ => {},
                }
            }

        },
        _ => {}
    }
}


// FIXME: Needs implementing
pub fn parse_mir(builder: &mut Attr, data: Vec<&BasicBlockData>) {
    //println!("\n\n\n{:#?}", builder);
    for index in 0..data.len() {
        //println!("bb{}", index);
        //println!("\n{:#?}-------------", data[index]);
    }

    wp(0, &data, builder);
}

// computes the weakest precondition
// FIXME: shouldnt return strings. Change to exression
// FIXME: move to wp module
fn wp(index: usize, data: &Vec<&BasicBlockData>, builder: &mut Attr) -> String {
    println!("\n\nExamining bb{:?}\n{:#?}", index, data[index]);

    // variables for tracking terminator
    let mut block_targets = Vec::new();
    let mut block_kind = "";

    // parse terminator data
    let terminator = data[index].terminator.clone().unwrap().kind;
    match terminator {
        TerminatorKind::Assert{cond, expected, msg, target, cleanup} => {
            // FIXME: look into
            //println!("{:#?}\n{:#?}\n{:#?}\n{:#?}\n{:#?}\n", cond, expected, msg, target, cleanup);
            block_targets.push(target);
            block_kind = "Assert";
        },
        TerminatorKind::Return => {
            // FIXME: shouldnt return string
            return "return".to_string()
        },
        TerminatorKind::Goto{target} => {
            block_targets.push(target);
            block_kind = "Goto";
        },
        TerminatorKind::Call{func, args, destination, cleanup} => unimplemented!(),
        TerminatorKind::DropAndReplace{location, value, target, unwind} => unimplemented!(),
        TerminatorKind::Drop{location, target, unwind} => unimplemented!(),
        TerminatorKind::Unreachable => unimplemented!(),
        TerminatorKind::Resume => unimplemented!(),
        TerminatorKind::If{cond, targets} => unimplemented!(),
        TerminatorKind::Switch{discr, adt_def, targets} => unimplemented!(),
        TerminatorKind::SwitchInt{discr, switch_ty, values, targets} => unimplemented!(),
        //_ => {}
    }


    // recurse to exit points of CFG and save return values
    // FIXME: save return data
    match block_kind {
        "Assert" |
        "Goto" => {
            let _ = wp(block_targets[0].index(), data, builder);
        },
        _ => {
            panic!("Unrecognized block kind");
        }
    }

    // FIXME: add wp generation

    // FIXME: remove
    return "".to_string();
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