#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(plugin_as_library)]

extern crate rustproof;

use rustproof::expression::{Expression, BinaryExpressionData, BinaryOperator};

fn main() {
	let x = 4;
	foo(x);
}

#[condition(pre="x: int > 0 AND x: int < 5", post="return: int > 5")]
fn foo(x: i32) -> i32 {
	x + 5
}