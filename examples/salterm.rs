#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(plugin_as_library)]

extern crate rustproof;

fn main() {
	let x: bool = true;
	valid_boolean_and_logical(x);
}

#[condition(pre="x:bool == true", post="return:bool == true")]
fn valid_boolean_and_logical(x:bool) -> bool {
    x && true
}
