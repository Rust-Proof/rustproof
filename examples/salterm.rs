#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(plugin_as_library)]

extern crate rustproof;

fn main() {
	let x: u64 = 4u64;
	let y: i64 = 4i64;
	foo(x);
	bar(x);
	baz(y);
}

// Invalid
#[condition(pre="x:u64 <= u64::MAX AND x:u64 >= u64::MIN", post="return:u64 == x:u64 + 5:u64")]
fn foo(x: u64) -> u64 {
		x + 5u64
}

// Valid
#[condition(pre="x:u64 <= u64::MAX - 5:u64 AND x:u64 >= u64::MIN", post="return:u64 == x:u64 + 5:u64")]
fn bar(x: u64) -> u64 {
		x + 5u64
}

// Signed integer test (currently gives incorrect results: always valid)
#[condition(pre="y:i64 <= i64::MAX AND y:i64 >= i64::MIN", post="return:i64 == y:i64 + 5:i64")]
fn baz(y: i64) -> i64 {
		y + 5i64
}