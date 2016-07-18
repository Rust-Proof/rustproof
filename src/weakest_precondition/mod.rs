// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use rustc_plugin::Registry;
use syntax::ast::{MetaItem, Item, ItemKind, MetaItemKind, LitKind, Attribute_};
use syntax::ext::base::{ExtCtxt, Annotatable};
use syntax::ext::base::SyntaxExtension::MultiDecorator;
use syntax::codemap::{Span, Spanned};
use syntax::parse::token::intern;
use syntax::ptr::P;
use super::dev_tools; // FIXME: remove for production
use super::Attr;
use super::expression;
use expression::Predicate;
use std::str::FromStr;
use rustc::mir::repr::{Mir, BasicBlock, BasicBlockData, TerminatorKind};
use rustc_data_structures::indexed_vec::Idx;
use super::parser;

// computes the weakest precondition
// FIXME: shouldnt return strings. Change to exression
// FIXME: move to wp module
// FIXME: do we want to return a predicate?
pub fn wp(index: usize, data: &Vec<&BasicBlockData>, builder: &mut Attr) -> Option<Predicate> {
    println!("\n\nExamining bb{:?}\n{:#?}", index, data[index]);

    // variables for tracking terminator
    // targets of terminator (exits)
    let mut block_targets = Vec::new();
    // bb kind
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
            // return the post condition as expression to start WP gen
            //return builder.post_expr.clone();
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
    let ret1: Option<Predicate> = None;
    match block_kind {
        "Assert" => {
            let ret1 = wp(block_targets[0].index(), data, builder);
        },
        "Goto" => {
            let ret1 = wp(block_targets[0].index(), data, builder);
        },
        _ => {
            panic!("Unrecognized block kind");
        }
    }

    // FIXME: add wp generation
    // examine statements in reverse order
    let mut stmts = data[index].statements.clone();
    stmts.reverse();
    for stmt in stmts {
        //process stmt into expression
        println!("{:?}", stmt);
    }

    // FIXME: not this prob
    return ret1;
}


//FIXME: wp is a predicate but is just a place holder for now. Will need appropriate type in
//       function arguments
pub fn stmt_substitution(wp: predicate, stmt: StatementKind)  {
    let var_to_sub = stmt.Lvalue;
    let sub_with = stmt.Rvalue;
    match var_to_sub {
        StatementKind::Lvalue::Var => {unimplemented!();}
        StatementKind::Lvalue::Temp => {unimplemented!();}
        StatementKind::Lvalue::Arg => {unimplemented!();}
        StatementKind::Lvalue::Static => {unimplemented!();}
        StatementKind::Lvalue::ReturnPointer => {unimplemented!();}
        StatementKind::Lvalue::Projection => {unimplemented!();}
    }
    match sub_with {
        StatementKind::Rvalue::CheckedBinaryOp => {unimplemented!();}
        StatementKind::Rvalue::BinaryOp => {unimplemented!();}
        StatementKind::Rvalue::UnaryOp => {unimplemented!();}
        _=> {unimplemented!();}
    }
}
