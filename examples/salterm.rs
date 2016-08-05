#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(plugin_as_library)]

extern crate rustproof;

use rustproof::expression::{Expression, BinaryExpressionData, BinaryOperator};

fn main() {
	let x = 4;
	foo(x);
}

#[condition(pre="true", post="true EQUIV true")]
fn foo(x: i32) {
	x + 1;
}