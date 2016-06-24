// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, and Drew Gohman.
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
#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]
// FIXME: these should not be here!
#![allow(unused_variables)]
#![allow(unused_imports)]

#[macro_use]
extern crate rustc;
extern crate syntax;
extern crate rustc_plugin;

pub mod reporting;
pub mod z3_interface;
pub mod weakest_precondition;
pub mod parser;
//pub mod data;

#[cfg(test)]
mod tests;

use rustc_plugin::Registry;
use syntax::ast::{MetaItem, Item, ItemKind, MetaItemKind};
use syntax::ext::base::{ExtCtxt, Annotatable};
use syntax::ext::base::SyntaxExtension::MultiDecorator;
use syntax::codemap::Span;
use syntax::parse::token::intern;
use syntax::ptr::P;

#[derive(Debug, Clone)]
pub struct Attr {
    // FIXME: super?
    pub pre: Option<syntax::ast::LitKind>,
    pub pre_str: String,
    pub post: Option<syntax::ast::LitKind>,
    pub post_str: String,
}

// Register plugin with compiler
#[plugin_registrar]
pub fn registrar(reg: &mut Registry) {
    reg.register_syntax_extension(intern("condition"), MultiDecorator(Box::new(expand_condition)));
}



// For every #[condition], this function is called
// FIXME: I don't really know what `push: &mut FnMut(Annotatable)` is, but I know its required.
fn expand_condition(ctx: &mut ExtCtxt, span: Span, meta: &MetaItem, item: &Annotatable, push: &mut FnMut(Annotatable)) {
    match item {
        &Annotatable::Item(ref it) => match it.node {
            // If the item is a function
            ItemKind::Fn(..) => {
                expand_condition_fn(meta);
            },
            // Otherwise, it shouldn't have #[condition] on it
            _ => expand_bad_item(ctx, span),
        },
        // If it isn't an item at all, also shouldn't have #[condition] on it
        _ => expand_bad_item(ctx, span),
    }
}


// If the #[condition] is on a function...
fn expand_condition_fn(meta: &MetaItem) {
    match meta.node {
        // FIXME: at the moment, error out if there are no arguments to the attribute
        MetaItemKind::List(ref attribute_name, ref args) => {
            // FIXME: arguments should be parsed by the parser module, not in this control module
            // NOTE: EXPERIMENT: control flow happens here
            let mut builder = Attr {pre: None, post: None, pre_str: "".to_string(), post_str: "".to_string()};
            parser::expand_args(&mut builder, args);
            println!("\nFINAL\n{:?}\n", builder);
        },
        _ => {
            panic!("Invalid arguments for #[condition]; did you add a pre and/or post condition?");
        }
    }
    //let () = meta.node;
}


// If the #[condition] is not on a function, error out
fn expand_bad_item(ctx: &mut ExtCtxt, span: Span) {
    ctx.span_err(span, "#[condition] must be placed on a function".into());
}
