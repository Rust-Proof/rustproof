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
use rustc::mir::repr::{Mir, BasicBlock, BasicBlockData, TerminatorKind};
use expression::Predicate;
use expression::Term;
use expression::AndData;
use expression::OrData;
use expression::ImpliesData;
use expression::IntegerComparisonData;
use expression::IntegerComparisonOperator;
use expression::SignedBitVectorData;
use std::str::FromStr;

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
    println!("\n\n\n{:#?}", builder);
    for index in 0..data.len() {
        println!("bb{}", index);
        println!("{:#?}", data[index]);
    }

    //WORK IN PROGRESS
    //wp(0, &data);

}

fn wp(index: usize, data: &Vec<&BasicBlockData>) -> Option<String> {
    let terminator = data[index].terminator.clone().unwrap().kind;
    //terminator.
    //println!("{:#?}", terminator);
    dev_tools::print_type_of(&terminator);
    match terminator {
        TerminatorKind::Assert{cond, expected, msg, target, cleanup} => unimplemented!(),
        TerminatorKind::Call{func, args, destination, cleanup} => unimplemented!(),
        TerminatorKind::DropAndReplace{location, value, target, unwind} => unimplemented!(),
        TerminatorKind::Drop{location, target, unwind} => unimplemented!(),
        TerminatorKind::Unreachable => unimplemented!(),
        TerminatorKind::Return => unimplemented!(),
        TerminatorKind::Resume => unimplemented!(),
        TerminatorKind::Goto{target} => unimplemented!(),
        TerminatorKind::If{cond, targets} => unimplemented!(),
        TerminatorKind::Switch{discr, adt_def, targets} => unimplemented!(),
        TerminatorKind::SwitchInt{discr, switch_ty, values, targets} => unimplemented!(),
    }
    return None;

}

pub fn real_parse(condition: &str) -> Term {
    match lalrpop::parse_T1(condition) {
        Ok(t) => {
            return t;
        },
        Err(e) => {
            panic!("Error parsing condition: {:?}", e);
        }
    }
}
