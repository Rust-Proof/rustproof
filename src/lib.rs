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
#![feature(macro_rules)]

// debug flag
const DEBUG: bool = true;

// External crate imports
extern crate env_logger;
#[macro_use] extern crate libsmt;
#[macro_use] extern crate log;
extern crate petgraph;
extern crate rustc;
extern crate rustc_plugin;
extern crate rustc_data_structures;
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
use rustc::ty::TyCtxt;
use syntax::ast::{MetaItem, Item, ItemKind, MetaItemKind};
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, Annotatable};
use syntax::ext::base::SyntaxExtension::MultiDecorator;
use syntax::parse::token::intern;
use syntax::ptr::P;

// Local imports
use expression::{Predicate, BooleanBinaryOperator, BinaryPredicateData};
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
mod tests; // Conditionally include tests when cargo --test is called

#[macro_use] pub mod reporting;

/*
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("printlns", printlns);
}
*/

#[derive(Debug)]
pub struct Attr {
    pub func_name: String,
    pub func_span: Option<Span>,
    pub func: Option<syntax::ptr::P<syntax::ast::Block>>,
    pub pre_span: Option<Span>,
    pub post_span: Option<Span>,
    pub pre_string: String,
    pub post_string: String,
    pub pre_expr: Option<Predicate>,
    pub post_expr: Option<Predicate>,
    pub weakest_precondition: Option<Predicate>,
}

impl Attr {
    fn clear(&mut self) {
        self.func_name = "".to_string();
        self.func_span = None;
        self.func = None;
        self.pre_span = None;
        self.post_span = None;
        self.pre_string = "".to_string();
        self.post_string = "".to_string();
        self.pre_expr = None;
        self.post_expr = None;
        self.weakest_precondition = None;
    }
}

// Register plugin with compiler
#[plugin_registrar]
pub fn registrar(reg: &mut Registry) {
	// This initializes the Reporting Module to Add the environment to the logger
    printlns!("This is a new test");
	reporting::init();
	//reg.register_syntax_extension(intern("condition"), MultiDecorator(Box::new(expand_condition)));
    //reg.register_syntax_extension(intern("condition"),
    //                              MultiDecorator(Box::new(expand_condition)));
    let visitor = MirVisitor {
        builder: Attr {
            func_name: "".to_string(),
            func_span: None,
            func: None,
            pre_string: "".to_string(),
            post_string: "".to_string(),
            pre_span: None,
            post_span: None,
            pre_expr: None,
            post_expr: None,
            weakest_precondition: None,
        },
    };
    reg.register_mir_pass(Box::new(visitor));
}

// FIXME: FOR NOW, THIS IS COMMENTED OUT FOR REFERENCE PURPOSES.
// For every #[condition], this function is called
// FIXME: I don't really know what `push: &mut FnMut(Annotatable)` is, but I know its required.
/// Checks an attribute for proper placement and starts the control flow of the application
/*
fn expand_condition(ctx: &mut ExtCtxt, span: Span,
                    meta: &MetaItem, item: &Annotatable,
                    push: &mut FnMut(Annotatable)) {
    match item {
        &Annotatable::Item(ref it) => match it.node {
            // If the item is a function
            ItemKind::Fn(..) => {
                control_flow(meta, item);
            },
            // Otherwise, it shouldn't have #[condition] on it
            _ => expand_bad_item(ctx, span),
        },
        // If it isn't an item at all, also shouldn't have #[condition] on it
        _ => expand_bad_item(ctx, span),
    }
}
*/

// FIXME: THIS WILL BE USED SOON
// If the #[condition] is not on a function, error out
/*
fn expand_bad_item(ctx: &mut ExtCtxt, span: Span) {
    ctx.span_err(span, "#[condition] must be placed on a function".into());
}
*/

struct MirVisitor {
    builder: Attr,
}

// This must be here, and it must be blank
impl <'tcx> Pass for MirVisitor {
}

impl <'tcx> MirPass<'tcx> for MirVisitor {
    fn run_pass<'a>(&mut self, tcx: TyCtxt<'a, 'tcx, 'tcx>, src: MirSource, mir: &mut Mir<'tcx>) {
        // Clear the stored attributes in the builder
        self.builder.clear();

        // Store relevant data
        let item_id = src.item_id();
        let def_id = tcx.map.local_def_id(item_id);
        let name = tcx.item_path_str(def_id);
        let attrs = tcx.map.attrs(item_id);

        self.builder.func_name = name;

        // FIXME: I'm pretty sure this is a bad way to do this. but it does work.
        for attr in attrs {
            parse_attribute(&mut self.builder, attr);
        }

        // FIXME: Better condition check
        if self.builder.pre_string != "" {
            // Parse the pre- and postcondition arguments
            self.builder.pre_expr = Some(parser::parse_condition(self.builder.pre_string.as_str()));
            self.builder.post_expr = Some(parser::parse_condition(self.builder.post_string.as_str()));

            // Begin examining the MIR code
            MirVisitor::visit_mir(self, mir);
        }

    }
}

impl<'tcx> Visitor<'tcx> for MirVisitor {
    // NOTE: Had trouble using visit_basic_block_data since I couldn't pass around the mir blocks.
    // This function instead implements visit_basic_block_data within it.
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
            // arg_data is a vector of ArgDecl
            arg_data.push(&mir.arg_decls[arg]);
        }
        // Get the temp declarations
        for index in 0..mir.temp_decls.len() {
            let temp = Temp::new(index);
            // temp_data is a vector of TempDecl
            temp_data.push(&mir.temp_decls[temp]);
        }
        // Get the variable declarations
        for index in 0..mir.var_decls.len() {
            let var = Var::new(index);
            // var_data is a vector of VarDecl
            var_data.push(&mir.var_decls[var]);
        }
        // Store all the data
        let data = (arg_data, block_data, temp_data, var_data);

        // Generate the weakest precondition
        self.builder.weakest_precondition = gen(0, &data, &self.builder);

        // Create the verification condition, P -> WP
        let verification_condition: Predicate = Predicate::BinaryExpression( BinaryPredicateData{
            op: BooleanBinaryOperator::Implication,
            p1: Box::new(self.builder.pre_expr.as_ref().unwrap().clone()),
            p2: Box::new(self.builder.weakest_precondition.as_ref().unwrap().clone())
        } );

        // FIXME: Remove debug print statement
        if DEBUG { println!("vc: {}\n", verification_condition); }

        // Output to smt_lib format
        //Pred2SMT::gen_smtlib(&verification_condition.clone());
        gen_smtlib(&verification_condition.clone());
    }
}
