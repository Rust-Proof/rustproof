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
use rustc::mir::repr::{Mir, BasicBlock, BasicBlockData, TerminatorKind, Statement, StatementKind, Lvalue, Rvalue};
use rustc_data_structures::indexed_vec::Idx;
use super::parser;

// computes the weakest precondition
pub fn gen(index: usize, data: &Vec<&BasicBlockData>, builder: &mut Attr) -> Option<Predicate> {
    println!("\n\nExamining bb{:?}\n{:#?}", index, data[index]);

    //let mut block_targets = Vec::new();
    //let mut block_kind = "";
    let mut wp: Option<Predicate> = None;

    // parse terminator data
    let terminator = data[index].terminator.clone().unwrap().kind;
    match terminator {
        TerminatorKind::Assert{cond, expected, msg, target, cleanup} => {
            wp = gen(target.index(), data, builder);
        },
        TerminatorKind::Return => {
            // return the post condition as expression to start WP gen
            wp = builder.post_expr.clone();
            println!("\nwp returned as\t{:?}\n", wp.clone().unwrap());
            return wp;
        },
        TerminatorKind::Goto{target} => {
            wp = gen(target.index(), data, builder);
        },
        TerminatorKind::Call{func, args, destination, cleanup} => unimplemented!(),
        TerminatorKind::DropAndReplace{location, value, target, unwind} => unimplemented!(),
        TerminatorKind::Drop{location, target, unwind} => unimplemented!(),
        TerminatorKind::Unreachable => unimplemented!(),
        TerminatorKind::Resume => unimplemented!(),
        TerminatorKind::If{cond, targets} => unimplemented!(),
        TerminatorKind::Switch{discr, adt_def, targets} => unimplemented!(),
        TerminatorKind::SwitchInt{discr, switch_ty, values, targets} => unimplemented!(),
    }



    // FIXME: add wp generation
    // examine statements in reverse order
    let mut stmts = data[index].statements.clone();
    stmts.reverse();
    for stmt in stmts {
        //process stmt into expression
        wp = gen_stmt(wp.unwrap(), stmt);
    }

    println!("\nwp returned as\t{:?}\n", wp.clone().unwrap());

    return wp;
}





//FIXME: wp is a predicate but is just a place holder for now. Will need appropriate type in
//       function arguments
pub fn gen_stmt(wp: Predicate, stmt: Statement) -> Option<Predicate>  {
    println!("processing statement\t{:?}\t\tinto predicate\t{:?}", stmt, wp);

    let mut lvalue: Option<Lvalue> = None;
    let mut rvalue: Option<Rvalue> = None;
    match stmt.kind {
        StatementKind::Assign(ref lval, ref rval) => {
            lvalue = Some(lval.clone());
            rvalue = Some(rval.clone());
        }
    }

    match lvalue.unwrap() {
        Lvalue::Var(ref var) => {unimplemented!();}
        Lvalue::Temp(ref temp) => {unimplemented!();}
        Lvalue::Arg(ref arg) => {unimplemented!();}
        Lvalue::Static(ref def_id) => {unimplemented!();}
        Lvalue::ReturnPointer => {unimplemented!();}
        Lvalue::Projection(_) => {panic!("wtf is a projection");}
    }

    match rvalue.unwrap() {
        Rvalue::CheckedBinaryOp(ref binop, ref lval, ref rval) => {unimplemented!();}
        Rvalue::BinaryOp(ref binop, ref lval, ref rval) => {unimplemented!();}
        Rvalue::UnaryOp(ref unop, ref val) => {unimplemented!();}
        _=> {panic!("Only CheckedBinaryOp, BinaryOp, and UnaryOp currently supported");}
    }


}
