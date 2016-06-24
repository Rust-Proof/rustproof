// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, and Drew Gohman.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//#[macro_use]
//extern crate rustc;
//extern crate syntax;
//extern crate rustc_plugin;

use rustc_plugin::Registry;
use syntax::ast::{MetaItem, Item, ItemKind, MetaItemKind, LitKind};
use syntax::ext::base::{ExtCtxt, Annotatable};
use syntax::ext::base::SyntaxExtension::MultiDecorator;
use syntax::codemap::Span;
use syntax::parse::token::intern;
use syntax::ptr::P;

// trash code [demo]
pub fn demo() {
    println!("parser - reporting in");
}



// FIXME: this should be in the parser module!
// Parse the condition arguments
pub fn expand_args(builder: &mut super::attr, args: &Vec<P<MetaItem>>) {

    match args.len() {
        1 => {
            panic!("You must provide pre AND post conditions.");
            /*
            println!("Found 1 argument:\n");
            println!("{:?}\n", args[0]);
            match args[0].node {
                MetaItemKind::NameValue(ref x, ref y) =>
                    {
                        if x=="pre" {
                            builder.pre = Some(y.node.clone());
                        } else if x=="post" {
                            builder.post = Some(y.node.clone());
                        } else {
                            panic!("expecting pre and/or post condition {} provided.", x);
                        }
                    },
                _ => {},
            }
            */
        },
        2 => {
            //println!("Found 2 arguments:\n");
            //println!("{:?}\n", args[0]);
            //println!("{:?}\n", args[1]);
            match args[0].node {
                MetaItemKind::NameValue(ref x, ref y) => {
                    if x!="pre" { panic!("The first argument must be 'pre'. {} was provided.", x); }
                    match y.node {
                        super::syntax::ast::LitKind::Str(ref x, ref y) => {
                            builder.pre_str = x.to_string();
                        }
                        _ => {}
                    }
                    //println!("\nDEBUG\n{:?}\n", y.node);
                    //let () = y.node;
                    //FIXME: future removal
                    builder.pre = Some(y.node.clone());
                },
                _ => {},
            }
            match args[1].node {
                MetaItemKind::NameValue(ref x, ref y) => {
                    if x!="post" { panic!("The second argument must be 'post'. {} was provided.", x); }
                    match y.node {
                        super::syntax::ast::LitKind::Str(ref x, ref y) => {
                            builder.post_str = x.to_string();
                        }
                        _ => {}
                    }
                    //FIXME: future removal?
                    builder.post = Some(y.node.clone());
                },
                _ => {},
            }
        },
        _ => {
            panic!("Too many arguments found for #[condition]; must have pre and/or post conditions");
        }
    }

    // FIXME: clone?
    //println!("precondition {:?}", builder.clone().pre.unwrap());
    //println!("postcondition {:?}", builder.clone().post.unwrap());
}
