#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(plugin_as_library)]

extern crate rustproof;

fn main() {
	let x = 4;
	foo(x);
}

#[condition(pre="x:u64 <= u64::MAX AND x:u64 >= u64::MIN", post="return:u64 == x:u64 + 5:u64")]
fn foo(x: u64) -> u64 {
	if (true) {
		x + 5
	} else {
		x + 5
	}
}