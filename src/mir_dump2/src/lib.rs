#![feature(plugin_registrar, rustc_private)]

#[macro_use]
extern crate rustc;
extern crate syntax;
extern crate rustc_plugin;

use rustc_plugin::Registry;
use syntax::ast::{MetaItem, Item, ItemKind};
use syntax::ext::base::{ExtCtxt, Annotatable};
use syntax::codemap::Span;
use syntax::ext::base::SyntaxExtension::MultiDecorator;
use syntax::parse::token::intern;

// Register plugin with the compiler
#[plugin_registrar]
pub fn registrar(reg: &mut Registry) {
    reg.register_syntax_extension(intern("precondition"), MultiDecorator(Box::new(expand_precondition)));
}

// For every #[precondition], this function is called
fn expand_precondition(ctx: &mut ExtCtxt, span: Span, meta: &MetaItem, item: &Annotatable, push: &mut FnMut(Annotatable)) {
    match item {
        &Annotatable::Item(ref inner_item) => match inner_item.node {
            ItemKind::Fn(..) => {
                expand_precondition_fn(meta);
            },
            _ => expand_bad_item(ctx, span),
        },
        _ => expand_bad_item(ctx, span),
    }
}

// If the #[precondition] is on a function... 
fn expand_precondition_fn(meta: &MetaItem) {
    println!("This is correctly placed on a function");
    println!("{:?}", meta);
    let data = statements::new();
    data::print();
}

// If the #[precondition] is not on a function, error out
fn expand_bad_item(ctx: &mut ExtCtxt, span: Span) {
    ctx.span_err(span, "#[precondition] must be placed over a function".into());
}
