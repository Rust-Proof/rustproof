#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(plugin_as_library)]

extern crate rustproof;

fn main() {
	let x: bool = true;
	foo(x);
	bar(x);
	baz(x);
}

// Valid
#[condition(pre="x: bool == true", post="return: bool == true")]
fn foo(x: bool) -> bool {
		x & true
}

// Valid
#[condition(pre="x: bool == false", post="return: bool == true")]
fn bar(x: bool) -> bool {
		x | true
}

// Valid
#[condition(pre="x: bool == false", post="return: bool == true")]
fn baz(x: bool) -> bool {
		x ^ true
}
