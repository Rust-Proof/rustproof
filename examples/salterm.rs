#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(plugin_as_library)]

extern crate rustproof;

fn main() {
	let x = 4;
	foo(x);
}

#[condition(pre="x:i32 > 0:i32 AND x:i32 < i32::MAX", post="return:i32 == x:i32 + 5:i32")]
fn foo(x: i32) -> i32 {
	x + 5
}