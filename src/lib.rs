// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Rustproof is a compiler plugin for the Rust programming language. It generates verification
//! conditions for functions with supplied preconditions(`P`) and postconditions(`Q`). That is, given a
//! supplied postcondition on a function, rustproof uses [predicate transformer semantics](https://en.wikipedia.org/wiki/Predicate_transformer_semantics)
//! to generate a weakest precondition(`WP`). The verification condition `P->WP` is then checked for
//! satisfiability by a SMT solver ([z3](https://github.com/Z3Prover/z3)).
//! This process results in a proof of function correctness.

#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]

#[macro_use] pub extern crate syntax;
#[macro_use] pub mod reporting;

// External crate imports
#[macro_use] extern crate libsmt;
#[macro_use] extern crate log;
extern crate petgraph;
extern crate rustc;
extern crate rustc_plugin;
extern crate rustc_data_structures;
extern crate rustc_const_math;
//extern crate syntax;
extern crate rustc_errors as errors;

// External imports
use rustc_data_structures::indexed_vec::Idx;
use rustc_plugin::Registry;
use rustc::mir::repr::{Mir, BasicBlock, BasicBlockData, Arg, Temp, Var, ArgDecl, TempDecl, VarDecl};
use rustc::mir::transform::{Pass, MirPass, MirSource};
use rustc::ty::{TyCtxt, FnOutput};
use syntax::feature_gate::AttributeType;
use syntax::parse::token::InternedString;
use syntax::ast::MetaItemKind;
use errors::{ColorConfig, Handler};
use syntax::codemap::CodeMap;
use std::rc::Rc;
use std::process;

// Local imports
use expression::{Expression, BinaryOperator, BinaryExpressionData};
use parser::*;
use smt_output::*;
use weakest_precondition::*;

// rustproof modules
mod expression;
mod parser;
mod smt_output;
mod weakest_precondition;
#[cfg(test)]
mod tests;

// Register plugin with compiler
#[plugin_registrar]
pub fn registrar(reg: &mut Registry) {
    // If debug is an argument, set the debug flag to true
    let mut debug = false;
    for arg in reg.args() {
        if arg.clone().unwrap().node == MetaItemKind::Word(InternedString::new("debug")) {
            debug = true;
        }
        else {
            rp_error!("unrecognized plugin argument");
        }
    }

    let visitor = MirVisitor { debug: debug };

    reg.register_attribute("condition".to_string(), AttributeType::Whitelisted);
    reg.register_mir_pass(Box::new(visitor));
}

/// Represents the data from the MirPass and parser_attributes functions
///
/// #Purpose:
/// *Used to pass data from the MIR and the computed weakest_precondition
///
pub struct MirData<'tcx> {
    block_data: Vec<&'tcx BasicBlockData<'tcx>>,
    arg_data: Vec<&'tcx ArgDecl<'tcx>>,
    var_data: Vec<&'tcx VarDecl<'tcx>>,
    temp_data: Vec<&'tcx TempDecl<'tcx>>,
    func_return_type: String,
}

// required struct for Pass impl
struct MirVisitor { debug: bool }

/// This must exsist and must be blank
impl <'tcx> Pass for MirVisitor {}

/// Sets up the compiler to go through MIR code.
///
/// # Remarks:
impl <'tcx> MirPass<'tcx> for MirVisitor {
    // Visit the MIR of the entire program
    fn run_pass<'a>(&mut self, tcx: TyCtxt<'a, 'tcx, 'tcx>, src: MirSource, mir: &mut Mir<'tcx>) {
        let debug = self.debug;
        // Clear the stored attributes in the builder
        let mut pre_string = "".to_string();
        let mut post_string = "".to_string();
        let pre_expr;
        let post_expr;

        // Store relevant data
        let item_id = src.item_id();
        let def_id = tcx.map.local_def_id(item_id);
        let name = tcx.item_path_str(def_id);
        let attrs = tcx.map.attrs(item_id);

        // TODO: Find a better way to do this
        for attr in attrs {
            parse_attribute(&mut pre_string, &mut post_string, attr);
        }

        // TODO: Find a better condition check
        if pre_string != "" {
            // Parse the pre- and postcondition arguments
            pre_expr = Some(parser::parse_condition(pre_string.as_str()));
            post_expr = Some(parser::parse_condition(post_string.as_str()));

            // Struct to carry MIR data to later stages
            let mut data = MirData {
                block_data: Vec::new(),
                arg_data: Vec::new(),
                var_data: Vec::new(),
                temp_data: Vec::new(),
                func_return_type: "".to_string(),
            };

            // Get the basic block data
            for index in 0..mir.basic_blocks().len() {
                let block = BasicBlock::new(index);
                data.block_data.push(&mir[block]);
            }

            // Get the function argument declarations
            for index in 0..mir.arg_decls.len() {
                let arg = Arg::new(index);
                data.arg_data.push(&mir.arg_decls[arg]);
            }

            // Get the temp declarations
            for index in 0..mir.temp_decls.len() {
                let temp = Temp::new(index);
                data.temp_data.push(&mir.temp_decls[temp]);
            }

            // Get the variable declarations
            for index in 0..mir.var_decls.len() {
                let var = Var::new(index);
                data.var_data.push(&mir.var_decls[var]);
            }

            // Get the return type
            data.func_return_type = match mir.return_ty {
                FnOutput::FnConverging(t) => {
                    t.to_string()
                },
                _ => { unimplemented!(); }
            };

            // Generate the weakest precondition
            let weakest_precondition = gen(0, &mut data, &post_expr, debug);

            // Create the verification condition, P -> WP
            let verification_condition: Expression = Expression::BinaryExpression( BinaryExpressionData{
                op: BinaryOperator::Implication,
                left: Box::new(pre_expr.as_ref().unwrap().clone()),
                right: Box::new(weakest_precondition.as_ref().unwrap().clone())
            } );

            // FIXME: Debug should not be a const; it must be user-facing
            if debug {
                println!("vc: {}\n", verification_condition);
            }

            // Output to SMT-LIB format
            gen_smtlib(&verification_condition.clone(), name, debug);
        }
    }
}
