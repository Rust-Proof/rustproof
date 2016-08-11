// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// TODO Refactor this code to follow rust guidelines
// https://github.com/rust-lang/rust/tree/master/src/doc/style

// These can be their own .rs file OR
// a named directory with mod.rs + other files
// see: https://doc.rust-lang.org/book/crates-and-modules.html
// see: 'tests' module (some things need pub that tests doesnt mind priv)
// see: 'reporting' module

// NOTE: Things to talk to rust devs about:
//     - Referencing lifetime stuff in struct that has impl
//     - Slice access; see line 143
//     - Unused attribute warnings since we aren't using register_syntax_extension
//          internals.rust-lang.org / users.rust-lang.org
//     - String to expression //libsyntax as parser / parse_exper_from_source_str
//
#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]
// FIXME: useful for development, delete when project is "complete"
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![feature(core_intrinsics)]
#![feature(libstd_sys_internals)]

#[macro_use] pub mod reporting;

// debug flag
const DEBUG: bool = false;

// External crate imports
extern crate env_logger;
#[macro_use] extern crate libsmt;
#[macro_use] extern crate log;
extern crate petgraph;
extern crate rustc;
extern crate rustc_plugin;
extern crate rustc_data_structures;
extern crate rustc_const_math;
extern crate syntax;
extern crate term;

// External imports
use env_logger::LogBuilder;
use log::{LogRecord, LogLevelFilter};
use rustc_data_structures::indexed_vec::Idx;
use rustc_plugin::Registry;
use rustc::mir::mir_map::MirMap;
use rustc::mir::repr::{Mir, BasicBlock, BasicBlockData, Arg, Temp, Var, ArgDecl, TempDecl, VarDecl};
use rustc::mir::transform::{Pass, MirPass, MirMapPass, MirSource, MirPassHook};
use rustc::mir::visit::Visitor;
use rustc::ty::{TyCtxt, FnOutput};
use syntax::ast::{MetaItem, Item, ItemKind, MetaItemKind};
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, Annotatable};
use syntax::ext::base::SyntaxExtension::MultiDecorator;
use syntax::parse::token::intern;
use syntax::ptr::P;

// Local imports
use expression::{Expression, BinaryOperator, BinaryExpressionData};
use parser::*;
use smt_output::*;
use weakest_precondition::*;

// These are our modules
pub mod expression;
pub mod parser;
pub mod smt_output;
pub mod weakest_precondition;
pub mod dev_tools; // FIXME: For debugging information, delete when project is "complete"
#[cfg(test)]
mod tests;



// Register plugin with compiler
#[plugin_registrar]
pub fn registrar(reg: &mut Registry) {
	// This initializes the Reporting Module to Add the environment to the logger
	reporting::init();

    let visitor = MirVisitor {
        pre_string: "".to_string(),
        post_string: "".to_string(),
        pre_expr: None,
        post_expr: None,
    };

    reg.register_mir_pass(Box::new(visitor));
}

struct MirVisitor {
    pre_string: String,
    post_string: String,
    pre_expr: Option<Expression>,
    post_expr: Option<Expression>,
}

// This must be here, and it must be blank
impl <'tcx> Pass for MirVisitor {}

impl <'tcx> MirPass<'tcx> for MirVisitor {
    // Visit the MIR of the entire program
    fn run_pass<'a>(&mut self, tcx: TyCtxt<'a, 'tcx, 'tcx>, src: MirSource, mir: &mut Mir<'tcx>) {
        // Clear the stored attributes in the builder
        self.pre_string = "".to_string();
        self.post_string = "".to_string();
        self.pre_expr = None;
        self.post_expr = None;

        // Store relevant data
        let item_id = src.item_id();
        let def_id = tcx.map.local_def_id(item_id);
        let name = tcx.item_path_str(def_id);
        let attrs = tcx.map.attrs(item_id);

        // TODO: Find a better way to do this
        for attr in attrs {
            parse_attribute(&mut self.pre_string, &mut self.post_string, attr);
        }

        // TODO: Find a better condition check
        if self.pre_string != "" {
            // Parse the pre- and postcondition arguments
            self.pre_expr = Some(parser::parse_condition(self.pre_string.as_str()));
            self.post_expr = Some(parser::parse_condition(self.post_string.as_str()));

            // Begin examining the MIR code
            MirVisitor::visit_mir(self, mir);
        }
    }
}

impl<'tcx> Visitor<'tcx> for MirVisitor {
    // Visit the MIR of a function
    fn visit_mir(&mut self, mir: &Mir<'tcx>) {
        let mut arg_data = Vec::new();
        let mut block_data = Vec::new();
        let mut temp_data = Vec::new();
        let mut var_data = Vec::new();

        // Get the basic block data
        for index in 0..mir.basic_blocks().len() {
            let block = BasicBlock::new(index);
            block_data.push(&mir[block]);
        }

        // Get the function argument declarations
        for index in 0..mir.arg_decls.len() {
            let arg = Arg::new(index);
            arg_data.push(&mir.arg_decls[arg]);
        }

        // Get the temp declarations
        for index in 0..mir.temp_decls.len() {
            let temp = Temp::new(index);
            temp_data.push(&mir.temp_decls[temp]);
        }

        // Get the variable declarations
        for index in 0..mir.var_decls.len() {
            let var = Var::new(index);
            var_data.push(&mir.var_decls[var]);
        }

        // Get the return type
        let func_return_type: String = match mir.return_ty {
            FnOutput::FnConverging(t) => {
                t.to_string()
            },
            _ => { unimplemented!(); }
        };

        // Store all the data
        let data = (arg_data, block_data, temp_data, var_data, func_return_type);

        // Generate the weakest precondition
        let weakest_precondition = gen(0, &data, &self.post_expr);

        // Create the verification condition, P -> WP
        let verification_condition: Expression = Expression::BinaryExpression( BinaryExpressionData{
            op: BinaryOperator::Implication,
            left: Box::new(self.pre_expr.as_ref().unwrap().clone()),
            right: Box::new(weakest_precondition.as_ref().unwrap().clone())
        } );

        // FIXME: Debug should not be a const; it must be user-facing
        if DEBUG { println!("vc: {}\n", verification_condition); }

        // Output to SMT-LIB format
        gen_smtlib(&verification_condition.clone());
    }
}
