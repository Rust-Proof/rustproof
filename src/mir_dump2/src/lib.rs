#![feature(plugin_registrar, rustc_private)]

#[macro_use]
extern crate rustc;
extern crate syntax;
extern crate rustc_plugin;

use rustc_plugin::Registry;

use syntax::ptr::P;
use syntax::ast::MetaItem;
use syntax::ext::base::{ExtCtxt, Annotatable};
use syntax::codemap::Span;
use syntax::ext::base::SyntaxExtension::MultiModifier;
use syntax::parse::token::intern;

#[plugin_registrar]
pub fn registrar(reg: &mut Registry) {
    reg.register_syntax_extension(intern("precondition"), MultiModifier(Box::new(expand)));
}

fn expand(ctx: &mut ExtCtxt, span: Span, meta: &MetaItem, ann: Annotatable) -> Annotatable {
    println!("hello world");
    return ann;
}
