// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// These can be their own .rs file OR
// a named directory with mod.rs + other files
// see: https://doc.rust-lang.org/book/crates-and-modules.html
// see: 'tests' module (some things need pub that tests doesnt mind priv)
// see: 'reporting' module

// NOTE: Things to talk to rust devs about:
//     - Referencing lifetime stuff in struct that has impl
//     - Slice access; see line 143
//     - Unused attribute warnings since we aren't using register_syntax_extension internals.rust-lang.org / users.rust-lang.org
//     - String to expression //libsyntax as parser / parse_exper_from_source_str
#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]
// FIXME: these should not be here!
#![allow(unused_variables)]
#![allow(unused_imports)]

// FIXME: remove below. only for dev tools
#![feature(core_intrinsics)]

#[macro_use]
extern crate rustc;
extern crate syntax;
extern crate rustc_plugin;
extern crate rustc_data_structures;

pub mod reporting;
pub mod z3_interface;
pub mod weakest_precondition;
pub mod parser;
pub mod dev_tools;
pub mod expression;
//pub mod data;

#[cfg(test)]
mod tests;

use rustc_data_structures::indexed_vec::Idx;

use rustc_plugin::Registry;
use syntax::ast::{MetaItem, Item, ItemKind, MetaItemKind};
use syntax::ext::base::{ExtCtxt, Annotatable};
use syntax::ext::base::SyntaxExtension::MultiDecorator;
use syntax::codemap::Span;
use syntax::parse::token::intern;
use syntax::ptr::P;
use expression::Predicate;

use rustc::mir::transform::{Pass, MirPass, MirMapPass, MirSource, MirPassHook};
use rustc::mir::mir_map::MirMap;
use rustc::mir::repr::{Mir, BasicBlock, BasicBlockData};
use rustc::mir::visit::Visitor;
use rustc::ty::TyCtxt;

#[derive(Debug)]
pub struct Attr {
    pub func_name: String,
    pub func_span: Option<Span>,
    pub func: Option<syntax::ptr::P<syntax::ast::Block>>,
    pub pre_span: Option<Span>,
    pub post_span: Option<Span>,
    pub pre_str: String,
    pub post_str: String,
    pub pre_expr: Option<Predicate>,
    pub post_expr: Option<Predicate>,
}

impl Attr {
    fn clear(&mut self) {
        self.func_name = "".to_string();
        self.func_span = None;
        self.func = None;
        self.pre_span = None;
        self.post_span = None;
        self.pre_str = "".to_string();
        self.post_str = "".to_string();
        self.pre_expr = None;
        self.post_expr = None;
    }
}

// Register plugin with compiler
#[plugin_registrar]
pub fn registrar(reg: &mut Registry) {
    //reg.register_syntax_extension(intern("condition"), MultiDecorator(Box::new(expand_condition)));
    let visitor = MirVisitor {
        builder: Attr {
            func_name: "".to_string(),
            func_span: None,
            func: None,
            pre_str: "".to_string(),
            post_str: "".to_string(),
            pre_span: None,
            post_span: None,
            pre_expr: None,
            post_expr: None,
        },
    };
    reg.register_mir_pass(Box::new(visitor));
}

// FIXME: FOR NOW, THIS IS COMMENTED OUT FOR REFERENCE PURPOSES.
// For every #[condition], this function is called
// FIXME: I don't really know what `push: &mut FnMut(Annotatable)` is, but I know its required.
/// Checks an attribute for proper placement and starts the control flow of the application
/*
fn expand_condition(ctx: &mut ExtCtxt, span: Span, meta: &MetaItem, item: &Annotatable, push: &mut FnMut(Annotatable)) {
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
        //clear the stored attributes in the builder
        self.builder.clear();
        
        let item_id = src.item_id();
        let def_id = tcx.map.local_def_id(item_id);
        let name = tcx.item_path_str(def_id);
        let attrs = tcx.map.attrs(item_id);

        self.builder.func_name = name;

        // FIXME: I'm pretty sure this is a bad way to do this. but it does work.
        for attr in attrs {
            parser::parse_attribute(&mut self.builder, attr);
        }

        if self.builder.pre_str != "" {
            println!("{}", parser::parse_condition(self.builder.pre_str.as_str()));
            self.builder.pre_expr = Some(parser::parse_condition(self.builder.pre_str.as_str()));
            println!("{}", parser::parse_condition(self.builder.post_str.as_str()));
            self.builder.post_expr = Some(parser::parse_condition(self.builder.post_str.as_str()));
            MirVisitor::visit_mir(self, mir);
        }
    }
}

impl<'tcx> Visitor<'tcx> for MirVisitor {
    // NOTE: Had trouble using visit_basic_block_data since I couldn't pass around the mir blocks.
    // This function instead implements visit_basic_block_data within it.
    fn visit_mir(&mut self, mir: &Mir<'tcx>) {
        let mut block_data = Vec::new();
        for index in 0..mir.basic_blocks().len() {
            let block = BasicBlock::new(index);
            block_data.push(&mir[block]);
        }
        parser::parse_mir(&mut self.builder, block_data);
    }
}
