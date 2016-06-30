#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]

extern crate rustproof;

use rustproof::expression::VariableMappingData;

fn main() {
	let x = 7u32;
	let z = foo(x);
}

#[condition(pre="b", post="c")]
fn foo(x: u32) -> u32 {
	let y = 5u32;
	x + y
}